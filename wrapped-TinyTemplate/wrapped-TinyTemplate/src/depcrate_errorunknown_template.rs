// Generated macro for unknown_template (function)
macro_rules! Depcrate_errorunknown_template {
() => {
// Module: crate::error
// Provides: {"unknown_template"}
// Dependencies: {}
pub (crate) fn unknown_template (source : & str , name : & str) -> Error { let (line , column) = get_offset (source , name) ; Error :: RenderError { msg : format ! ("Tried to call an unknown template '{}'" , name) , line , column , } }
};
}
