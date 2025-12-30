// Generated macro for impl_247 (impl)
macro_rules! Depcrate_inflateimpl_247 {
() => {
// Module: crate::inflate
// Provides: {"impl_247"}
// Dependencies: {}
impl TINFLStatus { pub fn from_i32 (value : i32) -> Option < TINFLStatus > { use self :: TINFLStatus :: * ; match value { TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS => Some (FailedCannotMakeProgress) , TINFL_STATUS_BAD_PARAM => Some (BadParam) , TINFL_STATUS_ADLER32_MISMATCH => Some (Adler32Mismatch) , TINFL_STATUS_FAILED => Some (Failed) , TINFL_STATUS_DONE => Some (Done) , TINFL_STATUS_NEEDS_MORE_INPUT => Some (NeedsMoreInput) , TINFL_STATUS_HAS_MORE_OUTPUT => Some (HasMoreOutput) , # [cfg (feature = "block-boundary")] TINFL_STATUS_BLOCK_BOUNDARY => Some (BlockBoundary) , _ => None , } } }
};
}
