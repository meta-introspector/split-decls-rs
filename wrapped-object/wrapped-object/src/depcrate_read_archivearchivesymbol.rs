// Generated macro for ArchiveSymbol (struct)
macro_rules! Depcrate_read_archiveArchiveSymbol {
() => {
// Module: crate::read::archive
// Provides: {"ArchiveSymbol"}
// Dependencies: {}
# [doc = " A symbol in the archive symbol table."] # [doc = ""] # [doc = " This is used to find the member containing the symbol."] # [derive (Debug , Clone , Copy)] pub struct ArchiveSymbol < 'data > { name : & 'data [u8] , offset : ArchiveOffset , }
};
}
