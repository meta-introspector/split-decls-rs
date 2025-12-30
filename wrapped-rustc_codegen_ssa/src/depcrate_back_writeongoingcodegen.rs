// Generated macro for OngoingCodegen (struct)
macro_rules! Depcrate_back_writeOngoingCodegen {
() => {
// Module: crate::back::write
// Provides: {"OngoingCodegen"}
// Dependencies: {}
pub struct OngoingCodegen < B : ExtraBackendMethods > { pub backend : B , pub crate_info : CrateInfo , pub output_filenames : Arc < OutputFilenames > , pub coordinator : Coordinator < B > , pub codegen_worker_receive : Receiver < CguMessage > , pub shared_emitter_main : SharedEmitterMain , }
};
}
