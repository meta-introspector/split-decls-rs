macro_rules! HasRecursiveOpaque {
    () => {
        struct HasRecursiveOpaque < 'a , 'tcx > { def_id : LocalDefId , seen : FxHashSet < LocalDefId > , opaques : & 'a FxIndexMap < LocalDefId , ty :: OpaqueHiddenType < 'tcx > > , tcx : TyCtxt < 'tcx > , }
    };
}

HasRecursiveOpaque!()