// Generated macro for impl_247 (impl)
macro_rules! Depcrate_check_wfcheckimpl_247 {
() => {
// Module: crate::check::wfcheck
// Provides: {"impl_247"}
// Dependencies: {}
impl < 'tcx > GATArgsCollector < 'tcx > { fn visit < T : TypeFoldable < TyCtxt < 'tcx > > > (gat : DefId , t : T ,) -> (FxIndexSet < (ty :: Region < 'tcx > , usize) > , FxIndexSet < (Ty < 'tcx > , usize) >) { let mut visitor = GATArgsCollector { gat , regions : FxIndexSet :: default () , types : FxIndexSet :: default () } ; t . visit_with (& mut visitor) ; (visitor . regions , visitor . types) } }
};
}
