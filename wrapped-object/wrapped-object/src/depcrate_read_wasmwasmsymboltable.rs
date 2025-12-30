// Generated macro for WasmSymbolTable (struct)
macro_rules! Depcrate_read_wasmWasmSymbolTable {
() => {
// Module: crate::read::wasm
// Provides: {"WasmSymbolTable"}
// Dependencies: {}
# [doc = " A symbol table in a [`WasmFile`]."] # [derive (Debug)] pub struct WasmSymbolTable < 'data , 'file > { symbols : & 'file [WasmSymbolInternal < 'data >] , }
};
}
