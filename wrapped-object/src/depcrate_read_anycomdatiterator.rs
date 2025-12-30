// Generated macro for ComdatIterator (struct)
macro_rules! Depcrate_read_anyComdatIterator {
() => {
// Module: crate::read::any
// Provides: {"ComdatIterator"}
// Dependencies: {}
# [doc = " An iterator for the COMDAT section groups in a [`File`]."] # [derive (Debug)] pub struct ComdatIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] > { inner : ComdatIteratorInternal < 'data , 'file , R > , }
};
}
