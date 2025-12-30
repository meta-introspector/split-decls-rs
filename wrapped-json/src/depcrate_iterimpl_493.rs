// Generated macro for impl_493 (impl)
macro_rules! Depcrate_iterimpl_493 {
() => {
// Module: crate::iter
// Provides: {"impl_493"}
// Dependencies: {}
impl < I > LineColIterator < I > where I : Iterator < Item = io :: Result < u8 > > , { pub fn new (iter : I) -> LineColIterator < I > { LineColIterator { iter , line : 1 , col : 0 , start_of_line : 0 , } } pub fn line (& self) -> usize { self . line } pub fn col (& self) -> usize { self . col } pub fn byte_offset (& self) -> usize { self . start_of_line + self . col } }
};
}
