// Generated macro for impl_141 (impl)
macro_rules! Depcrate_infer_free_regionsimpl_141 {
() => {
// Module: crate::infer::free_regions
// Provides: {"impl_141"}
// Dependencies: {}
impl < 'a , 'tcx > RegionRelations < 'a , 'tcx > { pub (crate) fn new (tcx : TyCtxt < 'tcx > , free_regions : & 'a FreeRegionMap < 'tcx >) -> Self { Self { tcx , free_regions } } pub (crate) fn lub_param_regions (& self , r_a : Region < 'tcx > , r_b : Region < 'tcx >) -> Region < 'tcx > { self . free_regions . lub_param_regions (self . tcx , r_a , r_b) } }
};
}
