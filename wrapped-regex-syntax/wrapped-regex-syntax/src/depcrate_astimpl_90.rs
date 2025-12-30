// Generated macro for impl_90 (impl)
macro_rules! Depcrate_astimpl_90 {
() => {
// Module: crate::ast
// Provides: {"impl_90"}
// Dependencies: {}
impl HexLiteralKind { # [doc = " The number of digits that must be used with this literal form when"] # [doc = " used without brackets. When used with brackets, there is no"] # [doc = " restriction on the number of digits."] pub fn digits (& self) -> u32 { match * self { HexLiteralKind :: X => 2 , HexLiteralKind :: UnicodeShort => 4 , HexLiteralKind :: UnicodeLong => 8 , } } }
};
}
