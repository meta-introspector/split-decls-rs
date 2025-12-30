// Generated macro for impl_903 (impl)
macro_rules! Depcrate_vtab_csvtabimpl_903 {
() => {
// Module: crate::vtab::csvtab
// Provides: {"impl_903"}
// Dependencies: {}
impl CsvTab { fn reader (& self) -> Result < csv :: Reader < File > , csv :: Error > { csv :: ReaderBuilder :: new () . has_headers (self . has_headers) . delimiter (self . delimiter) . quote (self . quote) . from_path (& self . filename) } fn parse_byte (arg : & str) -> Option < u8 > { if arg . len () == 1 { arg . bytes () . next () } else { None } } }
};
}
