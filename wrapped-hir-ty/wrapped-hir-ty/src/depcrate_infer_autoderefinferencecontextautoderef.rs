// Generated macro for InferenceContextAutoderef (type)
macro_rules! Depcrate_infer_autoderefInferenceContextAutoderef {
() => {
// Module: crate::infer::autoderef
// Provides: {"InferenceContextAutoderef"}
// Dependencies: {}
pub (crate) type InferenceContextAutoderef < 'a , 'b , 'db , Steps = Vec < (Ty < 'db > , AutoderefKind) > > = GeneralAutoderef < 'db , InferenceContextAutoderefCtx < 'a , 'b , 'db > , Steps > ;
};
}
