// Generated macro for impl_54 (impl)
macro_rules! Depcrate_stack_state_attributesimpl_54 {
() => {
// Module: crate::stack::state::attributes
// Provides: {"impl_54"}
// Dependencies: {}
impl Source { # [doc = " Returns non-worktree variants of `self` if `is_bare` is true."] pub fn adjust_for_bare (self , is_bare : bool) -> Self { if is_bare { Source :: IdMapping } else { self } } }
};
}
