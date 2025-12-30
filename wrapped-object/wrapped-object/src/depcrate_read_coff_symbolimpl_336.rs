// Generated macro for impl_336 (impl)
macro_rules! Depcrate_read_coff_symbolimpl_336 {
() => {
// Module: crate::read::coff::symbol
// Provides: {"impl_336"}
// Dependencies: {}
impl ImageSymbol for pe :: ImageSymbolEx { fn raw_name (& self) -> & [u8 ; 8] { & self . name } fn value (& self) -> u32 { self . value . get (LE) } fn section_number (& self) -> i32 { self . section_number . get (LE) } fn typ (& self) -> u16 { self . typ . get (LE) } fn storage_class (& self) -> u8 { self . storage_class } fn number_of_aux_symbols (& self) -> u8 { self . number_of_aux_symbols } }
};
}
