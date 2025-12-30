// Generated macro for submit_post_lto_module_to_llvm (function)
macro_rules! Depcrate_back_writesubmit_post_lto_module_to_llvm {
() => {
// Module: crate::back::write
// Provides: {"submit_post_lto_module_to_llvm"}
// Dependencies: {}
pub (crate) fn submit_post_lto_module_to_llvm < B : ExtraBackendMethods > (coordinator : & Coordinator < B > , module : CachedModuleCodegen ,) { let llvm_work_item = WorkItem :: CopyPostLtoArtifacts (module) ; drop (coordinator . sender . send (Message :: CodegenDone :: < B > { llvm_work_item , cost : 0 })) ; }
};
}
