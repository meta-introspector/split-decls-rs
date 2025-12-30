// Generated macro for impl_1373 (impl)
macro_rules! Depcrateimpl_1373 {
() => {
// Module: crate
// Provides: {"impl_1373"}
// Dependencies: {}
impl < 'tcx > ClosureOutlivesSubjectTy < 'tcx > { # [doc = " All regions of `ty` must be of kind `ReVar` and must represent"] # [doc = " universal regions *external* to the closure."] pub fn bind (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> Self { let inner = fold_regions (tcx , ty , | r , depth | match r . kind () { ty :: ReVar (vid) => { let br = ty :: BoundRegion { var : ty :: BoundVar :: from_usize (vid . index ()) , kind : ty :: BoundRegionKind :: Anon , } ; ty :: Region :: new_bound (tcx , depth , br) } _ => bug ! ("unexpected region in ClosureOutlivesSubjectTy: {r:?}") , }) ; Self { inner } } pub fn instantiate (self , tcx : TyCtxt < 'tcx > , mut map : impl FnMut (ty :: RegionVid) -> ty :: Region < 'tcx > ,) -> Ty < 'tcx > { fold_regions (tcx , self . inner , | r , depth | match r . kind () { ty :: ReBound (debruijn , br) => { debug_assert_eq ! (debruijn , depth) ; map (ty :: RegionVid :: from_usize (br . var . index ())) } _ => bug ! ("unexpected region {r:?}") , }) } }
};
}
