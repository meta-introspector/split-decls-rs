// Generated macro for FatLtoInput (enum)
macro_rules! Depcrate_back_writeFatLtoInput {
() => {
// Module: crate::back::write
// Provides: {"FatLtoInput"}
// Dependencies: {}
pub enum FatLtoInput < B : WriteBackendMethods > { Serialized { name : String , buffer : SerializedModule < B :: ModuleBuffer > } , InMemory (ModuleCodegen < B :: Module >) , }
};
}
