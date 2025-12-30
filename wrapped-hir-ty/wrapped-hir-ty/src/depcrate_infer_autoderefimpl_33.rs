// Generated macro for impl_33 (impl)
macro_rules! Depcrate_infer_autoderefimpl_33 {
() => {
// Module: crate::infer::autoderef
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'a , 'db > Autoderef < 'a , 'db > { # [inline] pub (crate) fn new_with_tracking (infcx : & 'a InferCtxt < 'db > , env : & 'a TraitEnvironment < 'db > , base_ty : Ty < 'db > ,) -> Self { Self :: new_impl (DefaultAutoderefCtx { infcx , env } , base_ty) } }
};
}
