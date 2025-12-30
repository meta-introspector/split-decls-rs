// Generated macro for SectionIterator (struct)
macro_rules! Depcrate_read_anySectionIterator {
() => {
// Module: crate::read::any
// Provides: {"SectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the sections in a [`File`]."] # [derive (Debug)] pub struct SectionIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] > { inner : SectionIteratorInternal < 'data , 'file , R > , }
};
}
