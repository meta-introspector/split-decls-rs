// Generated macro for impl_86 (impl)
macro_rules! Depcrate_contextimpl_86 {
() => {
// Module: crate::context
// Provides: {"impl_86"}
// Dependencies: {}
impl MacroLocation { pub fn from_location (location : & clang :: source :: SourceLocation < '_ >) -> Self { let clang :: source :: Location { file , line , column , offset , } = location . get_expansion_location () ; Self { file_id : file . map (| f | f . get_id ()) , line , column , offset , } } }
};
}
