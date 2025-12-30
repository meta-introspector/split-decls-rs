// Generated macro for CrelIteratorHeader (struct)
macro_rules! Depcrate_read_elf_relocationCrelIteratorHeader {
() => {
// Module: crate::read::elf::relocation
// Provides: {"CrelIteratorHeader"}
// Dependencies: {}
# [derive (Debug , Clone)] struct CrelIteratorHeader { # [doc = " The number of encoded relocations."] count : usize , # [doc = " The number of flag bits each relocation uses."] flag_bits : u64 , # [doc = " Shift of the relocation value."] shift : u64 , # [doc = " True if the relocation format encodes addend."] is_rela : bool , }
};
}
