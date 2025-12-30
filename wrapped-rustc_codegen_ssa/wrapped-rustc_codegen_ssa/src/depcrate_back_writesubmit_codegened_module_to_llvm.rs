// Generated macro for submit_codegened_module_to_llvm (function)
macro_rules! Depcrate_back_writesubmit_codegened_module_to_llvm {
() => {
// Module: crate::back::write
// Provides: {"submit_codegened_module_to_llvm"}
// Dependencies: {}
pub (crate) fn submit_codegened_module_to_llvm < B : ExtraBackendMethods > (coordinator : & Coordinator < B > , module : ModuleCodegen < B :: Module > , cost : u64 ,) { let llvm_work_item = WorkItem :: Optimize (module) ; drop (coordinator . sender . send (Message :: CodegenDone :: < B > { llvm_work_item , cost })) ; }
};
}
