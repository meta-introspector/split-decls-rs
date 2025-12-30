// Generated macro for WasmFile (struct)
macro_rules! Depcrate_read_wasmWasmFile {
() => {
// Module: crate::read::wasm
// Provides: {"WasmFile"}
// Dependencies: {}
# [doc = " A WebAssembly object file."] # [derive (Debug)] pub struct WasmFile < 'data , R = & 'data [u8] > { data : & 'data [u8] , has_memory64 : bool , sections : Vec < SectionHeader < 'data > > , id_sections : Box < [Option < usize > ; MAX_SECTION_ID + 1] > , has_debug_symbols : bool , symbols : Vec < WasmSymbolInternal < 'data > > , entry : u64 , marker : PhantomData < R > , }
};
}
