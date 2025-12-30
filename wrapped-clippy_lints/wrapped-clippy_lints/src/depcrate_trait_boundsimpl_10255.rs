// Generated macro for impl_10255 (impl)
macro_rules! Depcrate_trait_boundsimpl_10255 {
() => {
// Module: crate::trait_bounds
// Provides: {"impl_10255"}
// Dependencies: {}
impl PartialEq for ComparableTraitRef < '_ , '_ > { fn eq (& self , other : & Self) -> bool { SpanlessEq :: eq_modifiers (self . modifiers , other . modifiers) && SpanlessEq :: new (self . cx) . paths_by_resolution () . eq_path (self . trait_ref . path , other . trait_ref . path) } }
};
}
