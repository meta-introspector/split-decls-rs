macro_rules! deps {
    () => {
        ReportErrorExt!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl ReportErrorExt for ResourceExhaustionInfo { fn diagnostic_message (& self) -> DiagMessage { use crate :: fluent_generated :: * ; match self { ResourceExhaustionInfo :: StackFrameLimitReached => const_eval_stack_frame_limit_reached , ResourceExhaustionInfo :: MemoryExhausted => const_eval_memory_exhausted , ResourceExhaustionInfo :: AddressSpaceFull => const_eval_address_space_full , ResourceExhaustionInfo :: Interrupted => const_eval_interrupted , } } fn add_args < G : EmissionGuarantee > (self , _ : & mut Diag < '_ , G >) { } }
    };
}

impl_195!()