// Generated macro for impl_35 (impl)
macro_rules! Depcrate_infer_autoderefimpl_35 {
() => {
// Module: crate::infer::autoderef
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a , 'db > Autoderef < 'a , 'db , usize > { # [inline] pub (crate) fn new (infcx : & 'a InferCtxt < 'db > , env : & 'a TraitEnvironment < 'db > , base_ty : Ty < 'db > ,) -> Self { Self :: new_impl (DefaultAutoderefCtx { infcx , env } , base_ty) } }
};
}
