// Generated macro for check_place (function)
macro_rules! Depcrate_qualify_min_const_fncheck_place {
() => {
// Module: crate::qualify_min_const_fn
// Provides: {"check_place"}
// Dependencies: {}
fn check_place < 'tcx > (cx : & LateContext < 'tcx > , place : Place < 'tcx > , span : Span , body : & Body < 'tcx > , msrv : Msrv ,) -> McfResult { for (base , elem) in place . as_ref () . iter_projections () { match elem { ProjectionElem :: Field (..) => { if base . ty (body , cx . tcx) . ty . is_union () && ! msrv . meets (cx , msrvs :: CONST_FN_UNION) { return Err ((span , "accessing union fields is unstable" . into ())) ; } } , ProjectionElem :: Deref => match base . ty (body , cx . tcx) . ty . kind () { ty :: RawPtr (_ , hir :: Mutability :: Mut) => { return Err ((span , "dereferencing raw mut pointer in const fn is unstable" . into ())) ; } , ty :: RawPtr (_ , hir :: Mutability :: Not) if ! msrv . meets (cx , msrvs :: CONST_RAW_PTR_DEREF) => { return Err ((span , "dereferencing raw const pointer in const fn is unstable" . into ())) ; } , _ => () , } , ProjectionElem :: ConstantIndex { .. } | ProjectionElem :: OpaqueCast (..) | ProjectionElem :: Downcast (..) | ProjectionElem :: Subslice { .. } | ProjectionElem :: Index (_) | ProjectionElem :: UnwrapUnsafeBinder (_) => { } , } } Ok (()) }
};
}
