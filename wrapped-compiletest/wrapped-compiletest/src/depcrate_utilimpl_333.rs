// Generated macro for impl_333 (impl)
macro_rules! Depcrate_utilimpl_333 {
() => {
// Module: crate::util
// Provides: {"impl_333"}
// Dependencies: {}
impl Utf8PathBufExt for Utf8PathBuf { fn with_extra_extension (& self , extension : & str) -> Utf8PathBuf { if extension . is_empty () { self . clone () } else { let mut fname = self . file_name () . unwrap () . to_string () ; if ! extension . starts_with ('.') { fname . push_str (".") ; } fname . push_str (extension) ; self . with_file_name (fname) } } }
};
}
