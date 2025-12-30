// Generated macro for impl_1103 (impl)
macro_rules! Depcrate_read_wasmimpl_1103 {
() => {
// Module: crate::read::wasm
// Provides: {"impl_1103"}
// Dependencies: {}
impl < 'data , 'file > Iterator for WasmSymbolIterator < 'data , 'file > { type Item = WasmSymbol < 'data , 'file > ; fn next (& mut self) -> Option < Self :: Item > { let (index , symbol) = self . symbols . next () ? ; Some (WasmSymbol { index : SymbolIndex (index) , symbol , }) } }
};
}
