// Generated macro for AnnotateUnitFallbackVisitor (struct)
macro_rules! Depcrate_fallbackAnnotateUnitFallbackVisitor {
() => {
// Module: crate::fallback
// Provides: {"AnnotateUnitFallbackVisitor"}
// Dependencies: {}
# [doc = " Try to walk the HIR to find a place to insert a useful suggestion"] # [doc = " to preserve fallback to `()` in 2024."] struct AnnotateUnitFallbackVisitor < 'a , 'tcx > { reachable_vids : FxHashSet < ty :: TyVid > , fcx : & 'a FnCtxt < 'a , 'tcx > , }
};
}
