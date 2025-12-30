// Generated macro for truthiness_error (function)
macro_rules! Depcrate_errortruthiness_error {
() => {
// Module: crate::error
// Provides: {"truthiness_error"}
// Dependencies: {}
pub (crate) fn truthiness_error (source : & str , path : PathSlice) -> Error { let (line , column) = get_offset (source , path . last () . unwrap ()) ; Error :: RenderError { msg : format ! ("Path '{}' produced a value which could not be checked for truthiness." , path_to_str (path)) , line , column , } }
};
}
