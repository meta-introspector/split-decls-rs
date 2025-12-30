// Generated macro for Der (struct)
macro_rules! DepcrateDer {
() => {
// Module: crate
// Provides: {"Der"}
// Dependencies: {}
# [doc = " DER-encoded data, either owned or borrowed"] # [doc = ""] # [doc = " This wrapper type is used to represent DER-encoded data in a way that is agnostic to whether"] # [doc = " the data is owned (by a `Vec<u8>`) or borrowed (by a `&[u8]`). Support for the owned"] # [doc = " variant is only available when the `alloc` feature is enabled."] # [derive (Clone , Eq , Hash , PartialEq)] pub struct Der < 'a > (BytesInner < 'a >) ;
};
}
