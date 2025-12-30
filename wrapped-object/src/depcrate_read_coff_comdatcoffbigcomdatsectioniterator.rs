// Generated macro for CoffBigComdatSectionIterator (type)
macro_rules! Depcrate_read_coff_comdatCoffBigComdatSectionIterator {
() => {
// Module: crate::read::coff::comdat
// Provides: {"CoffBigComdatSectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the sections in a COMDAT section group in a [`CoffBigFile`](super::CoffBigFile)."] pub type CoffBigComdatSectionIterator < 'data , 'file , R = & 'data [u8] > = CoffComdatSectionIterator < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
};
}
