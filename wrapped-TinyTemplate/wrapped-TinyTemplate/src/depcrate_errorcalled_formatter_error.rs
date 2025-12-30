// Generated macro for called_formatter_error (function)
macro_rules! Depcrate_errorcalled_formatter_error {
() => {
// Module: crate::error
// Provides: {"called_formatter_error"}
// Dependencies: {}
pub (crate) fn called_formatter_error (source : & str , formatter_name : & str , err : Error) -> Error { let (line , column) = get_offset (source , formatter_name) ; Error :: CalledFormatterError { name : formatter_name . to_string () , err : Box :: new (err) , line , column , } }
};
}
