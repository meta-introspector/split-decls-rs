// Generated macro for increase_arc_refcount (function)
macro_rules! Depcrate_symbolincrease_arc_refcount {
() => {
// Module: crate::symbol
// Provides: {"increase_arc_refcount"}
// Dependencies: {}
fn increase_arc_refcount (repr : TaggedArcPtr) -> TaggedArcPtr { let Some (arc) = (unsafe { repr . try_as_arc_owned () }) else { return repr ; } ; mem :: forget (Arc :: clone (& arc)) ; repr }
};
}
