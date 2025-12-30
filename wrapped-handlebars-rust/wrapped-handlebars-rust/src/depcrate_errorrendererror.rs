// Generated macro for RenderError (struct)
macro_rules! Depcrate_errorRenderError {
() => {
// Module: crate::error
// Provides: {"RenderError"}
// Dependencies: {}
# [doc = " Error when rendering data on template."] # [non_exhaustive] # [derive (Debug)] pub struct RenderError { pub template_name : Option < String > , pub line_no : Option < usize > , pub column_no : Option < usize > , reason : Box < RenderErrorReason > , unimplemented : bool , }
};
}
