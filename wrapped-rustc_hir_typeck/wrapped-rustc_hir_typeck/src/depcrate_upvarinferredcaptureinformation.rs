// Generated macro for InferredCaptureInformation (type)
macro_rules! Depcrate_upvarInferredCaptureInformation {
() => {
// Module: crate::upvar
// Provides: {"InferredCaptureInformation"}
// Dependencies: {}
# [doc = " Intermediate format to store a captured `Place` and associated `ty::CaptureInfo`"] # [doc = " during capture analysis. Information in this map feeds into the minimum capture"] # [doc = " analysis pass."] type InferredCaptureInformation < 'tcx > = Vec < (Place < 'tcx > , ty :: CaptureInfo) > ;
};
}
