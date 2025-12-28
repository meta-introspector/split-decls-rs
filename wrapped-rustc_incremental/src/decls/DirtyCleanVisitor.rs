macro_rules! DirtyCleanVisitor {
    () => {
        struct DirtyCleanVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , checked_attrs : FxHashSet < ast :: AttrId > , }
    };
}

DirtyCleanVisitor!();