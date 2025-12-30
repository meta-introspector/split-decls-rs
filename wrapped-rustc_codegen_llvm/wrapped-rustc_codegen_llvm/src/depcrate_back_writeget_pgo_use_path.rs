// Generated macro for get_pgo_use_path (function)
macro_rules! Depcrate_back_writeget_pgo_use_path {
() => {
// Module: crate::back::write
// Provides: {"get_pgo_use_path"}
// Dependencies: {}
fn get_pgo_use_path (config : & ModuleConfig) -> Option < CString > { config . pgo_use . as_ref () . map (| path_buf | CString :: new (path_buf . to_string_lossy () . as_bytes ()) . unwrap ()) }
};
}
