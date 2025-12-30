// Generated macro for impl_92 (impl)
macro_rules! Depcrate_errorimpl_92 {
() => {
// Module: crate::error
// Provides: {"impl_92"}
// Dependencies: {}
impl From < RenderErrorReason > for RenderError { fn from (e : RenderErrorReason) -> RenderError { RenderError { template_name : None , line_no : None , column_no : None , reason : Box :: new (e) , unimplemented : false , } } }
};
}
