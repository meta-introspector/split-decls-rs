// Generated macro for called_template_error (function)
macro_rules! Depcrate_errorcalled_template_error {
() => {
// Module: crate::error
// Provides: {"called_template_error"}
// Dependencies: {}
pub (crate) fn called_template_error (source : & str , template_name : & str , err : Error) -> Error { let (line , column) = get_offset (source , template_name) ; Error :: CalledTemplateError { name : template_name . to_string () , err : Box :: new (err) , line , column , } }
};
}
