// Generated macro for impl_130 (impl)
macro_rules! Depcrate_astimpl_130 {
() => {
// Module: crate::ast
// Provides: {"impl_130"}
// Dependencies: {}
impl FlagsItemKind { # [doc = " Returns true if and only if this item is a negation operator."] pub fn is_negation (& self) -> bool { match * self { FlagsItemKind :: Negation => true , _ => false , } } }
};
}
