macro_rules! FindClosureArg {
    () => {
        struct FindClosureArg < 'tcx > { tcx : TyCtxt < 'tcx > , calls : Vec < (& 'tcx hir :: Expr < 'tcx > , & 'tcx [hir :: Expr < 'tcx >]) > , }
    };
}

FindClosureArg!();