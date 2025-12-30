// Generated macro for impl_378 (impl)
macro_rules! Depcrate_errorsimpl_378 {
() => {
// Module: crate::errors
// Provides: {"impl_378"}
// Dependencies: {}
impl ReportErrorExt for ResourceExhaustionInfo { fn diagnostic_message (& self) -> DiagMessage { use crate :: fluent_generated :: * ; match self { ResourceExhaustionInfo :: StackFrameLimitReached => const_eval_stack_frame_limit_reached , ResourceExhaustionInfo :: MemoryExhausted => const_eval_memory_exhausted , ResourceExhaustionInfo :: AddressSpaceFull => const_eval_address_space_full , ResourceExhaustionInfo :: Interrupted => const_eval_interrupted , } } fn add_args < G : EmissionGuarantee > (self , _ : & mut Diag < '_ , G >) { } }
};
}
