// Generated macro for LeakCheck (struct)
macro_rules! Depcrate_infer_region_constraints_leak_checkLeakCheck {
() => {
// Module: crate::infer::region_constraints::leak_check
// Provides: {"LeakCheck"}
// Dependencies: {}
struct LeakCheck < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , outer_universe : ty :: UniverseIndex , mini_graph : MiniGraph < 'tcx > , rcc : RegionConstraintCollector < 'a , 'tcx > , scc_placeholders : IndexVec < LeakCheckScc , Option < ty :: PlaceholderRegion > > , scc_universes : IndexVec < LeakCheckScc , SccUniverse < 'tcx > > , }
};
}
