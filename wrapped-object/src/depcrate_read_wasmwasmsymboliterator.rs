// Generated macro for WasmSymbolIterator (struct)
macro_rules! Depcrate_read_wasmWasmSymbolIterator {
() => {
// Module: crate::read::wasm
// Provides: {"WasmSymbolIterator"}
// Dependencies: {}
# [doc = " An iterator for the symbols in a [`WasmFile`]."] # [derive (Debug)] pub struct WasmSymbolIterator < 'data , 'file > { symbols : core :: iter :: Enumerate < slice :: Iter < 'file , WasmSymbolInternal < 'data > > > , }
};
}
