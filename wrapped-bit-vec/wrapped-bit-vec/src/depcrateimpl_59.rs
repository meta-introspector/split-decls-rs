// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'a , B : BitBlock > Iterator for IterMut < 'a , B > { type Item = MutBorrowedBit < 'a , B > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let index = self . range . next () ; self . get (index) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
};
}
