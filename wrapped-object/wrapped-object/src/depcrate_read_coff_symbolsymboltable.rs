// Generated macro for SymbolTable (struct)
macro_rules! Depcrate_read_coff_symbolSymbolTable {
() => {
// Module: crate::read::coff::symbol
// Provides: {"SymbolTable"}
// Dependencies: {}
# [doc = " A table of symbol entries in a COFF or PE file."] # [doc = ""] # [doc = " Also includes the string table used for the symbol names."] # [doc = ""] # [doc = " Returned by [`CoffHeader::symbols`] and"] # [doc = " [`ImageNtHeaders::symbols`](crate::read::pe::ImageNtHeaders::symbols)."] # [derive (Debug)] pub struct SymbolTable < 'data , R = & 'data [u8] , Coff = pe :: ImageFileHeader > where R : ReadRef < 'data > , Coff : CoffHeader , { symbols : & 'data [Coff :: ImageSymbolBytes] , strings : StringTable < 'data , R > , }
};
}
