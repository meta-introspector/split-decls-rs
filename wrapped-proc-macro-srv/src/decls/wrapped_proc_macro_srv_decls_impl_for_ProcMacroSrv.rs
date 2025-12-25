use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ProcMacroSrv<'_> {
    pub fn expand<S: ProcMacroSrvSpan>(
        &self,
        lib: impl AsRef<Utf8Path>,
        env: &[(String, String)],
        current_dir: Option<impl AsRef<Path>>,
        macro_name: &str,
        macro_body: tt::TopSubtree<S>,
        attribute: Option<tt::TopSubtree<S>>,
        def_site: S,
        call_site: S,
        mixed_site: S,
    ) -> Result<Vec<tt::TokenTree<S>>, PanicMessage> {
        let snapped_env = self.env;
        let expander = self.expander(lib.as_ref()).map_err(|err| PanicMessage {
            message: Some(format!("failed to load macro: {err}")),
        })?;
        let prev_env = EnvChange::apply(snapped_env, env, current_dir.as_ref().map(<_>::as_ref));
        let result = thread::scope(|s| {
            let thread = thread::Builder::new()
                .stack_size(EXPANDER_STACK_SIZE)
                .name(macro_name.to_owned())
                .spawn_scoped(s, move || {
                    expander
                        .expand(
                            macro_name,
                            server_impl::TopSubtree(macro_body.0.into_vec()),
                            attribute.map(|it| server_impl::TopSubtree(it.0.into_vec())),
                            def_site,
                            call_site,
                            mixed_site,
                        )
                        .map(|tt| tt.0)
                });
            match thread.unwrap().join() {
                Ok(res) => res,
                Err(e) => std::panic::resume_unwind(e),
            }
        });
        prev_env.rollback();
        result
    }
    pub fn list_macros(
        &self,
        dylib_path: &Utf8Path,
    ) -> Result<Vec<(String, ProcMacroKind)>, String> {
        let expander = self.expander(dylib_path)?;
        Ok(expander
            .list_macros()
            .map(|(k, v)| (k.to_owned(), v))
            .collect())
    }
    fn expander(&self, path: &Utf8Path) -> Result<Arc<dylib::Expander>, String> {
        let expander = || {
            let expander = dylib::Expander::new(&self.temp_dir, path)
                .map_err(|err| format!("Cannot create expander for {path}: {err}",));
            expander.map(Arc::new)
        };
        Ok(
            match self
                .expanders
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .entry(path.to_path_buf())
            {
                Entry::Vacant(v) => v.insert(expander()?).clone(),
                Entry::Occupied(mut e) => {
                    let time = fs::metadata(path).and_then(|it| it.modified()).ok();
                    if Some(e.get().modified_time()) != time {
                        e.insert(expander()?);
                    }
                    e.get().clone()
                }
            },
        )
    }
}
