// Generated macro for impl_34 (impl)
macro_rules! Depcrate_infer_autoderefimpl_34 {
() => {
// Module: crate::infer::autoderef
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'a , 'b , 'db > InferenceContextAutoderef < 'a , 'b , 'db > { # [inline] pub (crate) fn new_from_inference_context (ctx : & 'a mut InferenceContext < 'b , 'db > , base_ty : Ty < 'db > ,) -> Self { Self :: new_impl (InferenceContextAutoderefCtx (ctx) , base_ty) } # [inline] pub (crate) fn ctx (& mut self) -> & mut InferenceContext < 'b , 'db > { self . ctx . 0 } }
};
}
