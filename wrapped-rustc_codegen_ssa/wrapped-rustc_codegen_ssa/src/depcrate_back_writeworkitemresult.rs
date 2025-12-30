// Generated macro for WorkItemResult (enum)
macro_rules! Depcrate_back_writeWorkItemResult {
() => {
// Module: crate::back::write
// Provides: {"WorkItemResult"}
// Dependencies: {}
# [doc = " A result produced by the backend."] pub (crate) enum WorkItemResult < B : WriteBackendMethods > { # [doc = " The backend has finished compiling a CGU, nothing more required."] Finished (CompiledModule) , # [doc = " The backend has finished compiling a CGU, which now needs to go through"] # [doc = " fat LTO."] NeedsFatLto (FatLtoInput < B >) , # [doc = " The backend has finished compiling a CGU, which now needs to go through"] # [doc = " thin LTO."] NeedsThinLto (String , B :: ThinBuffer) , }
};
}
