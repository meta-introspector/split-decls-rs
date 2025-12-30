// Generated macro for CoffBigComdatIterator (type)
macro_rules! Depcrate_read_coff_comdatCoffBigComdatIterator {
() => {
// Module: crate::read::coff::comdat
// Provides: {"CoffBigComdatIterator"}
// Dependencies: {}
# [doc = " An iterator for the COMDAT section groups in a [`CoffBigFile`](super::CoffBigFile)."] pub type CoffBigComdatIterator < 'data , 'file , R = & 'data [u8] > = CoffComdatIterator < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
};
}
