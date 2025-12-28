macro_rules! deps {
    () => {
        FnCtxt!();
    };
}

macro_rules! AnnotateUnitFallbackVisitor {
    () => {
        deps!();
        # [doc = " Try to walk the HIR to find a place to insert a useful suggestion"] # [doc = " to preserve fallback to `()` in 2024."] struct AnnotateUnitFallbackVisitor < 'a , 'tcx > { reachable_vids : FxHashSet < ty :: TyVid > , fcx : & 'a FnCtxt < 'a , 'tcx > , }
    };
}

AnnotateUnitFallbackVisitor!();