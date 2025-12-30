// Generated macro for impl_356 (impl)
macro_rules! Depcrate_groupbylazyimpl_356 {
() => {
// Module: crate::groupbylazy
// Provides: {"impl_356"}
// Dependencies: {}
impl < K , I , F > Debug for GroupInner < K , I , F > where K : Debug , I : Iterator + Debug , I :: Item : Debug , { debug_fmt_fields ! (GroupInner , iter , current_key , current_elt , done , top_group , oldest_buffered_group , bottom_group , buffer , dropped_group) ; }
};
}
