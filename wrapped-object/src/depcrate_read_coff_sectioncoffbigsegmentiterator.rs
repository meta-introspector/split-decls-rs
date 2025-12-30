// Generated macro for CoffBigSegmentIterator (type)
macro_rules! Depcrate_read_coff_sectionCoffBigSegmentIterator {
() => {
// Module: crate::read::coff::section
// Provides: {"CoffBigSegmentIterator"}
// Dependencies: {}
# [doc = " An iterator for the loadable sections in a [`CoffBigFile`](super::CoffBigFile)."] pub type CoffBigSegmentIterator < 'data , 'file , R = & 'data [u8] > = CoffSegmentIterator < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
};
}
