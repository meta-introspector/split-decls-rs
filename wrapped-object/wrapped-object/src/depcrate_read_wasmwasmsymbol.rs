// Generated macro for WasmSymbol (struct)
macro_rules! Depcrate_read_wasmWasmSymbol {
() => {
// Module: crate::read::wasm
// Provides: {"WasmSymbol"}
// Dependencies: {}
# [doc = " A symbol in a [`WasmFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSymbol`] trait implementation."] # [derive (Clone , Copy , Debug)] pub struct WasmSymbol < 'data , 'file > { index : SymbolIndex , symbol : & 'file WasmSymbolInternal < 'data > , }
};
}
