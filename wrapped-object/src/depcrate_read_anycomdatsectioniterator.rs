// Generated macro for ComdatSectionIterator (struct)
macro_rules! Depcrate_read_anyComdatSectionIterator {
() => {
// Module: crate::read::any
// Provides: {"ComdatSectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the sections in a [`Comdat`]."] # [derive (Debug)] pub struct ComdatSectionIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] > { inner : ComdatSectionIteratorInternal < 'data , 'file , R > , }
};
}
