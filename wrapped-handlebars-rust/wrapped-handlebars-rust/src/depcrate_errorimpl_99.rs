// Generated macro for impl_99 (impl)
macro_rules! Depcrate_errorimpl_99 {
() => {
// Module: crate::error
// Provides: {"impl_99"}
// Dependencies: {}
impl From < (IOError , String) > for TemplateError { fn from (err_info : (IOError , String)) -> TemplateError { let (e , name) = err_info ; TemplateError :: of (TemplateErrorReason :: IoError (e , name)) } }
};
}
