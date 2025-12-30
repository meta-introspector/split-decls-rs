// Generated macro for impl_552 (impl)
macro_rules! Depcrate_read_listsimpl_552 {
() => {
// Module: crate::read::lists
// Provides: {"impl_552"}
// Dependencies: {}
impl ListsHeader { # [doc = " Return the serialized size of the table header."] # [allow (dead_code)] # [inline] fn size (self) -> u8 { ListsHeader :: size_for_encoding (self . encoding) } # [doc = " Return the serialized size of the table header."] # [inline] pub (crate) fn size_for_encoding (encoding : Encoding) -> u8 { encoding . format . initial_length_size () + 2 + 1 + 1 + 4 } }
};
}
