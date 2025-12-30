// Generated macro for WasmSectionIterator (struct)
macro_rules! Depcrate_read_wasmWasmSectionIterator {
() => {
// Module: crate::read::wasm
// Provides: {"WasmSectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the sections in a [`WasmFile`]."] # [derive (Debug)] pub struct WasmSectionIterator < 'data , 'file , R = & 'data [u8] > { file : & 'file WasmFile < 'data , R > , sections : slice :: Iter < 'file , SectionHeader < 'data > > , }
};
}
