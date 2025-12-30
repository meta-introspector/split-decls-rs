// Generated macro for CoffBigSectionIterator (type)
macro_rules! Depcrate_read_coff_sectionCoffBigSectionIterator {
() => {
// Module: crate::read::coff::section
// Provides: {"CoffBigSectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the sections in a [`CoffBigFile`](super::CoffBigFile)."] pub type CoffBigSectionIterator < 'data , 'file , R = & 'data [u8] > = CoffSectionIterator < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
};
}
