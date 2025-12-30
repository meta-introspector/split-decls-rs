// Generated macro for impl_3807 (impl)
macro_rules! Depcrate_loops_utilsimpl_3807 {
() => {
// Module: crate::loops::utils
// Provides: {"impl_3807"}
// Dependencies: {}
impl < 'a , 'tcx > IncrementVisitor < 'a , 'tcx > { pub (super) fn new (cx : & 'a LateContext < 'tcx >) -> Self { Self { cx , states : HirIdMap :: default () , depth : 0 , } } pub (super) fn into_results (self) -> impl Iterator < Item = HirId > { self . states . into_iter () . filter_map (| (id , state) | { if state == IncrementVisitorVarState :: IncrOnce { Some (id) } else { None } }) } }
};
}
