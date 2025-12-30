// Generated macro for CoffBigRelocationIterator (type)
macro_rules! Depcrate_read_coff_relocationCoffBigRelocationIterator {
() => {
// Module: crate::read::coff::relocation
// Provides: {"CoffBigRelocationIterator"}
// Dependencies: {}
# [doc = " An iterator for the relocations in a [`CoffBigSection`](super::CoffBigSection)."] pub type CoffBigRelocationIterator < 'data , 'file , R = & 'data [u8] > = CoffRelocationIterator < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
};
}
