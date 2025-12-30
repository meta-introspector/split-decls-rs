// Generated macro for VisitWith (type)
macro_rules! Depcrate_provenance_gcVisitWith {
() => {
// Module: crate::provenance_gc
// Provides: {"VisitWith"}
// Dependencies: {}
pub type VisitWith < 'a > = dyn FnMut (Option < AllocId > , Option < BorTag >) + 'a ;
};
}
