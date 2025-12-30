// Generated macro for impl_38 (impl)
macro_rules! Depcrate_astimpl_38 {
() => {
// Module: crate::ast
// Provides: {"impl_38"}
// Dependencies: {}
impl GenericArgs { pub fn is_angle_bracketed (& self) -> bool { matches ! (self , AngleBracketed (..)) } pub fn span (& self) -> Span { match self { AngleBracketed (data) => data . span , Parenthesized (data) => data . span , ParenthesizedElided (span) => * span , } } }
};
}
