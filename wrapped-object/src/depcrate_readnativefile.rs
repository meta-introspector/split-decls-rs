// Generated macro for NativeFile (type)
macro_rules! Depcrate_readNativeFile {
() => {
// Module: crate::read
// Provides: {"NativeFile"}
// Dependencies: {}
# [doc = " The native executable file for the target platform."] # [cfg (all (feature = "wasm" , target_arch = "wasm32" , feature = "wasm"))] pub type NativeFile < 'data , R = & 'data [u8] > = wasm :: WasmFile < 'data , R > ;
};
}
