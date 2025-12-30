// Generated macro for impl_132 (impl)
macro_rules! Depcrate_parser_tokenimpl_132 {
() => {
// Module: crate::parser::token
// Provides: {"impl_132"}
// Dependencies: {}
impl < 's , P > From < (& 's str , bool) > for ParsedPatternItem < 's , P > { fn from (input : (& 's str , bool)) -> Self { Self :: Literal { content : Cow :: Borrowed (input . 0) , quoted : input . 1 , } } }
};
}
