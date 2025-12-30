// Generated macro for DynamicRelocationIterator (struct)
macro_rules! Depcrate_read_anyDynamicRelocationIterator {
() => {
// Module: crate::read::any
// Provides: {"DynamicRelocationIterator"}
// Dependencies: {}
# [doc = " An iterator for the dynamic relocation entries in a [`File`]."] # [derive (Debug)] pub struct DynamicRelocationIterator < 'data , 'file , R = & 'data [u8] > where R : ReadRef < 'data > , { inner : DynamicRelocationIteratorInternal < 'data , 'file , R > , }
};
}
