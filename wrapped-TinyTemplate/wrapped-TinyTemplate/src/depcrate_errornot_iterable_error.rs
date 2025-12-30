// Generated macro for not_iterable_error (function)
macro_rules! Depcrate_errornot_iterable_error {
() => {
// Module: crate::error
// Provides: {"not_iterable_error"}
// Dependencies: {}
pub (crate) fn not_iterable_error (source : & str , path : PathSlice) -> Error { let (line , column) = get_offset (source , path . last () . unwrap ()) ; Error :: RenderError { msg : format ! ("Expected an array for path '{}' but found a non-iterable value." , path_to_str (path)) , line , column , } }
};
}
