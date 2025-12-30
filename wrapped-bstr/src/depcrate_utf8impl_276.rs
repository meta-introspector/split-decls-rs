// Generated macro for impl_276 (impl)
macro_rules! Depcrate_utf8impl_276 {
() => {
// Module: crate::utf8
// Provides: {"impl_276"}
// Dependencies: {}
impl < 'a > CharIndices < 'a > { pub (crate) fn new (bs : & 'a [u8]) -> CharIndices < 'a > { CharIndices { bs , forward_index : 0 , reverse_index : bs . len () } } # [doc = " View the underlying data as a subslice of the original data."] # [doc = ""] # [doc = " The slice returned has the same lifetime as the original slice, and so"] # [doc = " the iterator can continue to be used while this exists."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::ByteSlice;"] # [doc = ""] # [doc = " let mut it = b\"abc\".char_indices();"] # [doc = ""] # [doc = " assert_eq!(b\"abc\", it.as_bytes());"] # [doc = " it.next();"] # [doc = " assert_eq!(b\"bc\", it.as_bytes());"] # [doc = " it.next();"] # [doc = " it.next();"] # [doc = " assert_eq!(b\"\", it.as_bytes());"] # [doc = " ```"] # [inline] pub fn as_bytes (& self) -> & 'a [u8] { self . bs } }
};
}
