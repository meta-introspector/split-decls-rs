// Generated macro for unknown_formatter (function)
macro_rules! Depcrate_errorunknown_formatter {
() => {
// Module: crate::error
// Provides: {"unknown_formatter"}
// Dependencies: {}
pub (crate) fn unknown_formatter (source : & str , name : & str) -> Error { let (line , column) = get_offset (source , name) ; Error :: RenderError { msg : format ! ("Tried to call an unknown formatter '{}'" , name) , line , column , } }
};
}
