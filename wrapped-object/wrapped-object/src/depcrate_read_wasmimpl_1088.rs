// Generated macro for impl_1088 (impl)
macro_rules! Depcrate_read_wasmimpl_1088 {
() => {
// Module: crate::read::wasm
// Provides: {"impl_1088"}
// Dependencies: {}
impl < 'data , 'file , R > Iterator for WasmSectionIterator < 'data , 'file , R > { type Item = WasmSection < 'data , 'file , R > ; fn next (& mut self) -> Option < Self :: Item > { let section = self . sections . next () ? ; Some (WasmSection { file : self . file , section , }) } }
};
}
