// Generated macro for impl_68 (impl)
macro_rules! Depcrate_errorimpl_68 {
() => {
// Module: crate::error
// Provides: {"impl_68"}
// Dependencies: {}
impl StringStreamError { fn as_str (& self) -> & str { use self :: StringStreamError :: * ; match * self { UnexpectedParse => "unexpected parse" , Eoi => "unexpected end of input" , CharacterBoundary => CHAR_BOUNDARY_ERROR_MESSAGE , } } }
};
}
