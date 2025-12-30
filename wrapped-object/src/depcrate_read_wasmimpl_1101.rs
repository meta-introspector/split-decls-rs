// Generated macro for impl_1101 (impl)
macro_rules! Depcrate_read_wasmimpl_1101 {
() => {
// Module: crate::read::wasm
// Provides: {"impl_1101"}
// Dependencies: {}
impl < 'data , 'file > ObjectSymbolTable < 'data > for WasmSymbolTable < 'data , 'file > { type Symbol = WasmSymbol < 'data , 'file > ; type SymbolIterator = WasmSymbolIterator < 'data , 'file > ; fn symbols (& self) -> Self :: SymbolIterator { WasmSymbolIterator { symbols : self . symbols . iter () . enumerate () , } } fn symbol_by_index (& self , index : SymbolIndex) -> Result < Self :: Symbol > { let symbol = self . symbols . get (index . 0) . read_error ("Invalid Wasm symbol index") ? ; Ok (WasmSymbol { index , symbol }) } }
};
}
