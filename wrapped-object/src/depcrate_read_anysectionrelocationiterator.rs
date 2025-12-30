// Generated macro for SectionRelocationIterator (struct)
macro_rules! Depcrate_read_anySectionRelocationIterator {
() => {
// Module: crate::read::any
// Provides: {"SectionRelocationIterator"}
// Dependencies: {}
# [doc = " An iterator for the relocation entries in a [`Section`]."] # [derive (Debug)] pub struct SectionRelocationIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] > { inner : SectionRelocationIteratorInternal < 'data , 'file , R > , }
};
}
