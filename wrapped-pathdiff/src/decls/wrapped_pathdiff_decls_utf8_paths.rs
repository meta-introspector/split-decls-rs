use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "camino")]
mod utf8_paths {
    use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
    /// Construct a relative UTF-8 path from a provided base directory path to the provided path.
    ///
    /// Requires the `camino` feature.
    ///
    /// ```rust
    /// # extern crate camino;
    /// use camino::*;
    /// use pathdiff::diff_utf8_paths;
    ///
    /// assert_eq!(diff_utf8_paths("/foo/bar",      "/foo/bar/baz"),  Some("../".into()));
    /// assert_eq!(diff_utf8_paths("/foo/bar/baz",  "/foo/bar"),      Some("baz".into()));
    /// assert_eq!(diff_utf8_paths("/foo/bar/quux", "/foo/bar/baz"),  Some("../quux".into()));
    /// assert_eq!(diff_utf8_paths("/foo/bar/baz",  "/foo/bar/quux"), Some("../baz".into()));
    /// assert_eq!(diff_utf8_paths("/foo/bar",      "/foo/bar/quux"), Some("../".into()));
    ///
    /// assert_eq!(diff_utf8_paths("/foo/bar",      "baz"),           Some("/foo/bar".into()));
    /// assert_eq!(diff_utf8_paths("/foo/bar",      "/baz"),          Some("../foo/bar".into()));
    /// assert_eq!(diff_utf8_paths("foo",           "bar"),           Some("../foo".into()));
    ///
    /// assert_eq!(
    ///     diff_utf8_paths(&"/foo/bar/baz", "/foo/bar".to_string()),
    ///     Some("baz".into())
    /// );
    /// assert_eq!(
    ///     diff_utf8_paths(Utf8Path::new("/foo/bar/baz"), Utf8Path::new("/foo/bar").to_path_buf()),
    ///     Some("baz".into())
    /// );
    /// ```
    #[cfg_attr(docsrs, doc(cfg(feature = "camino")))]
    pub fn diff_utf8_paths<P, B>(path: P, base: B) -> Option<Utf8PathBuf>
    where
        P: AsRef<Utf8Path>,
        B: AsRef<Utf8Path>,
    {
        let path = path.as_ref();
        let base = base.as_ref();
        if path.is_absolute() != base.is_absolute() {
            if path.is_absolute() { Some(Utf8PathBuf::from(path)) } else { None }
        } else {
            let mut ita = path.components();
            let mut itb = base.components();
            let mut comps: Vec<Utf8Component> = vec![];
            if let Some(Utf8Component::CurDir) = ita.clone().next() {
                ita.next();
            }
            if let Some(Utf8Component::CurDir) = itb.clone().next() {
                itb.next();
            }
            loop {
                match (ita.next(), itb.next()) {
                    (None, None) => break,
                    (Some(a), None) => {
                        comps.push(a);
                        comps.extend(ita.by_ref());
                        break;
                    }
                    (None, _) => comps.push(Utf8Component::ParentDir),
                    (Some(a), Some(b)) if comps.is_empty() && a == b => {}
                    (Some(a), Some(b)) if b == Utf8Component::CurDir => comps.push(a),
                    (Some(_), Some(b)) if b == Utf8Component::ParentDir => return None,
                    (Some(a), Some(_)) => {
                        comps.push(Utf8Component::ParentDir);
                        for _ in itb {
                            comps.push(Utf8Component::ParentDir);
                        }
                        comps.push(a);
                        comps.extend(ita.by_ref());
                        break;
                    }
                }
            }
            Some(comps.iter().map(|c| c.as_str()).collect())
        }
    }
}
