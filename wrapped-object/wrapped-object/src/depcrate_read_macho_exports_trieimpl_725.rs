// Generated macro for impl_725 (impl)
macro_rules! Depcrate_read_macho_exports_trieimpl_725 {
() => {
// Module: crate::read::macho::exports_trie
// Provides: {"impl_725"}
// Dependencies: {}
impl < 'data > ExportSymbol < 'data > { # [doc = " The name of the exported symbol."] pub fn name (& self) -> & [u8] { & self . name } # [doc = " The flags for the exported symbol."] pub fn flags (& self) -> u8 { self . flags } # [doc = " The terminal data for the exported symbol."] pub fn data (& self) -> & ExportData < 'data > { & self . data } }
};
}
