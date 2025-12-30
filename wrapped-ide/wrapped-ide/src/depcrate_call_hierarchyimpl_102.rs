// Generated macro for impl_102 (impl)
macro_rules! Depcrate_call_hierarchyimpl_102 {
() => {
// Module: crate::call_hierarchy
// Provides: {"impl_102"}
// Dependencies: {}
impl CallLocations { fn add (& mut self , target : NavigationTarget , range : FileRange) { self . funcs . entry (target) . or_default () . push (range) ; } fn into_items (self) -> Vec < CallItem > { self . funcs . into_iter () . map (| (target , ranges) | CallItem { target , ranges }) . collect () } }
};
}
