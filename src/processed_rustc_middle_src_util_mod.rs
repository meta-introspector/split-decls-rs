// SRC: ../rust/compiler/rustc_middle/src/util/mod.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=Providers | COMPLEXITY=2 | LINES=8 */

#[derive(Default, Copy, Clone)]
pub struct Providers {
    pub queries: crate::query::Providers,
    pub extern_queries: crate::query::ExternProviders,
    pub hooks: crate::hooks::Providers,
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=deref_mut | COMPLEXITY=5 | LINES=9 */

/// Backwards compatibility hack to keep the diff small. This
/// gives direct access to the `queries` field's fields, which
/// are what almost everything wants access to.
impl std::ops::DerefMut for Providers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queries
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=deref | COMPLEXITY=5 | LINES=8 */

impl std::ops::Deref for Providers {
    type Target = crate::query::Providers;

    fn deref(&self) -> &Self::Target {
        &self.queries
    }
}