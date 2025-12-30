// Generated macro for impl_533 (impl)
macro_rules! Depcrate_read_elf_relocationimpl_533 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"impl_533"}
// Dependencies: {}
impl < 'data > Iterator for CrelIterator < 'data > { type Item = read :: Result < Crel > ; fn next (& mut self) -> Option < Self :: Item > { if self . state . index >= self . header . count { return None ; } let result = self . parse () ; if result . is_err () { self . state . index = self . header . count ; } Some (result) } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
};
}
