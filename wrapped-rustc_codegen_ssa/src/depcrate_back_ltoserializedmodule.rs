// Generated macro for SerializedModule (enum)
macro_rules! Depcrate_back_ltoSerializedModule {
() => {
// Module: crate::back::lto
// Provides: {"SerializedModule"}
// Dependencies: {}
pub enum SerializedModule < M : ModuleBufferMethods > { Local (M) , FromRlib (Vec < u8 >) , FromUncompressedFile (Mmap) , }
};
}
