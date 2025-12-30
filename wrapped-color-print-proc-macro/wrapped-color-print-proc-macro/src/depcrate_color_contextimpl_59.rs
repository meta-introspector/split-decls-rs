// Generated macro for impl_59 (impl)
macro_rules! Depcrate_color_contextimpl_59 {
() => {
// Module: crate::color_context
// Provides: {"impl_59"}
// Dependencies: {}
impl < T > Action < T > where T : PartialEq , { # [doc = " Creates a new [`Action`]."] pub fn from_diff (old : Option < T > , new : Option < T >) -> Self { let eq = old == new ; match (old , new , eq) { (Some (old_val) , Some (_) , true) | (Some (old_val) , None , _) => Action :: Keep (old_val) , (_ , Some (new_val) , _) => Action :: Change (new_val) , _ => Action :: None , } } }
};
}
