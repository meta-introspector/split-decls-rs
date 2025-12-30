// Generated macro for base_local_and_movability (function)
macro_rules! Depcrate_redundant_clonebase_local_and_movability {
() => {
// Module: crate::redundant_clone
// Provides: {"base_local_and_movability"}
// Dependencies: {}
# [doc = " Extracts and returns the undermost base `Local` of given `place`. Returns `place` itself"] # [doc = " if it is already a `Local`."] # [doc = ""] # [doc = " Also reports whether given `place` cannot be moved out."] fn base_local_and_movability < 'tcx > (cx : & LateContext < 'tcx > , mir : & mir :: Body < 'tcx > , place : mir :: Place < 'tcx > ,) -> (mir :: Local , CannotMoveOut) { let mut deref = false ; let mut field = false ; let mut slice = false ; for (base , elem) in place . as_ref () . iter_projections () { let base_ty = base . ty (& mir . local_decls , cx . tcx) . ty ; deref |= matches ! (elem , mir :: ProjectionElem :: Deref) ; field |= matches ! (elem , mir :: ProjectionElem :: Field (..)) && has_drop (cx , base_ty) ; slice |= matches ! (elem , mir :: ProjectionElem :: Index (..)) && ! is_copy (cx , base_ty) ; } (place . local , deref || field || slice) }
};
}
