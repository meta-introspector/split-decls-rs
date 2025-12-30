// Generated macro for impl_28 (impl)
macro_rules! Depcrate_infer_autoderefimpl_28 {
() => {
// Module: crate::infer::autoderef
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'db > AutoderefCtx < 'db > for InferenceContextAutoderefCtx < '_ , '_ , 'db > { # [inline] fn infcx (& self) -> & InferCtxt < 'db > { & self . 0 . table . infer_ctxt } # [inline] fn env (& self) -> & TraitEnvironment < 'db > { & self . 0 . table . trait_env } }
};
}
