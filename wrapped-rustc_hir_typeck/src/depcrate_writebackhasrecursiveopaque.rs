// Generated macro for HasRecursiveOpaque (struct)
macro_rules! Depcrate_writebackHasRecursiveOpaque {
() => {
// Module: crate::writeback
// Provides: {"HasRecursiveOpaque"}
// Dependencies: {}
struct HasRecursiveOpaque < 'a , 'tcx > { def_id : LocalDefId , seen : FxHashSet < LocalDefId > , opaques : & 'a FxIndexMap < LocalDefId , ty :: OpaqueHiddenType < 'tcx > > , tcx : TyCtxt < 'tcx > , }
};
}
