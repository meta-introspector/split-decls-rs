// Generated macro for impl_77 (impl)
macro_rules! Depcrate_astimpl_77 {
() => {
// Module: crate::ast
// Provides: {"impl_77"}
// Dependencies: {}
impl ByRef { # [must_use] pub fn cap_ref_mutability (mut self , mutbl : Mutability) -> Self { if let ByRef :: Yes (old_mutbl) = & mut self { * old_mutbl = cmp :: min (* old_mutbl , mutbl) ; } self } }
};
}
