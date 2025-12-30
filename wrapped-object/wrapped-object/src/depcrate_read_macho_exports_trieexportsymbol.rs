// Generated macro for ExportSymbol (struct)
macro_rules! Depcrate_read_macho_exports_trieExportSymbol {
() => {
// Module: crate::read::macho::exports_trie
// Provides: {"ExportSymbol"}
// Dependencies: {}
# [doc = " Exported symbol information."] # [derive (Debug)] pub struct ExportSymbol < 'data > { name : Box < [u8] > , flags : u8 , data : ExportData < 'data > , }
};
}
