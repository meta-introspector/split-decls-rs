// Generated macro for impl_38 (impl)
macro_rules! Depcrate_file_listimpl_38 {
() => {
// Module: crate::file_list
// Provides: {"impl_38"}
// Dependencies: {}
impl From < web_sys :: FileList > for FileList { fn from (raw : web_sys :: FileList) -> Self { let length = raw . length () ; let inner = (0 .. length) . map (| i | File :: from (raw . get (i) . unwrap_throw ())) . collect () ; FileList { inner } } }
};
}
