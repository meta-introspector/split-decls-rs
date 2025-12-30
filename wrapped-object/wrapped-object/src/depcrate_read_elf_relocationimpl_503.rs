// Generated macro for impl_503 (impl)
macro_rules! Depcrate_read_elf_relocationimpl_503 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"impl_503"}
// Dependencies: {}
impl < 'data , Elf : FileHeader > Iterator for ElfRelocationIterator < 'data , Elf > { type Item = Crel ; fn next (& mut self) -> Option < Self :: Item > { match self { ElfRelocationIterator :: Rel (ref mut i , endian) => { i . next () . map (| r | Crel :: from_rel (r , * endian)) } ElfRelocationIterator :: Rela (ref mut i , endian , is_mips64el) => { i . next () . map (| r | Crel :: from_rela (r , * endian , * is_mips64el)) } ElfRelocationIterator :: Crel (ref mut i) => i . next () . and_then (Result :: ok) , } } }
};
}
