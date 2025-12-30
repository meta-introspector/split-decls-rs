// Generated macro for impl_142 (impl)
macro_rules! Depcrate_contextimpl_142 {
() => {
// Module: crate::context
// Provides: {"impl_142"}
// Dependencies: {}
impl PathCompletionCtx < '_ > { pub (crate) fn is_trivial_path (& self) -> bool { matches ! (self , PathCompletionCtx { has_call_parens : false , has_macro_bang : false , qualified : Qualified :: No , parent : None , has_type_args : false , .. }) } }
};
}
