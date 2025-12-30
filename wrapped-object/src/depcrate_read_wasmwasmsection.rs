// Generated macro for WasmSection (struct)
macro_rules! Depcrate_read_wasmWasmSection {
() => {
// Module: crate::read::wasm
// Provides: {"WasmSection"}
// Dependencies: {}
# [doc = " A section in a [`WasmFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSection`] trait implementation."] # [derive (Debug)] pub struct WasmSection < 'data , 'file , R = & 'data [u8] > { file : & 'file WasmFile < 'data , R > , section : & 'file SectionHeader < 'data > , }
};
}
