// Generated macro for impl_221 (impl)
macro_rules! Depcrate_read_anyimpl_221 {
() => {
// Module: crate::read::any
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > > Iterator for DynamicRelocationIterator < 'data , 'file , R > { type Item = (u64 , Relocation) ; fn next (& mut self) -> Option < Self :: Item > { match self . inner { # [cfg (feature = "elf")] DynamicRelocationIteratorInternal :: Elf32 (ref mut elf) => elf . next () , # [cfg (feature = "elf")] DynamicRelocationIteratorInternal :: Elf64 (ref mut elf) => elf . next () , DynamicRelocationIteratorInternal :: None (_) => None , } } }
};
}
