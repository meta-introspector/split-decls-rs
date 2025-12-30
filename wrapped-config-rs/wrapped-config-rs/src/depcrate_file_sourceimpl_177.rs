// Generated macro for impl_177 (impl)
macro_rules! Depcrate_file_sourceimpl_177 {
() => {
// Module: crate::file::source
// Provides: {"impl_177"}
// Dependencies: {}
impl FileSourceResult { pub fn uri (& self) -> & Option < String > { & self . uri } pub fn content (& self) -> & str { self . content . as_str () } pub fn format (& self) -> & dyn Format { self . format . as_ref () } }
};
}
