// Generated macro for RelocationIterator (struct)
macro_rules! Depcrate_read_pe_relocationRelocationIterator {
() => {
// Module: crate::read::pe::relocation
// Provides: {"RelocationIterator"}
// Dependencies: {}
# [doc = " An iterator of the relocations in a block in the `.reloc` section of a PE file."] # [derive (Debug , Clone)] pub struct RelocationIterator < 'data > { virtual_address : u32 , size : u32 , relocs : slice :: Iter < 'data , U16 < LE > > , }
};
}
