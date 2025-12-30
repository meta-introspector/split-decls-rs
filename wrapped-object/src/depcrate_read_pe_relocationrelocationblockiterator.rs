// Generated macro for RelocationBlockIterator (struct)
macro_rules! Depcrate_read_pe_relocationRelocationBlockIterator {
() => {
// Module: crate::read::pe::relocation
// Provides: {"RelocationBlockIterator"}
// Dependencies: {}
# [doc = " An iterator over the relocation blocks in the `.reloc` section of a PE file."] # [doc = ""] # [doc = " Returned by [`DataDirectories::relocation_blocks`](super::DataDirectories::relocation_blocks)."] # [derive (Debug , Default , Clone , Copy)] pub struct RelocationBlockIterator < 'data > { data : Bytes < 'data > , }
};
}
