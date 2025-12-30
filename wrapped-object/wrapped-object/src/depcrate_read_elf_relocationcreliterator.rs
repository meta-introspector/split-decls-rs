// Generated macro for CrelIterator (struct)
macro_rules! Depcrate_read_elf_relocationCrelIterator {
() => {
// Module: crate::read::elf::relocation
// Provides: {"CrelIterator"}
// Dependencies: {}
# [doc = " Compact relocation iterator."] # [derive (Debug , Clone)] pub struct CrelIterator < 'data > { # [doc = " Input stream reader."] data : Bytes < 'data > , # [doc = " Parsed header information."] header : CrelIteratorHeader , # [doc = " State of the iterator."] state : CrelIteratorState , }
};
}
