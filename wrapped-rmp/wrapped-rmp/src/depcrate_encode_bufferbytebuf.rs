// Generated macro for ByteBuf (struct)
macro_rules! Depcrate_encode_bufferByteBuf {
() => {
// Module: crate::encode::buffer
// Provides: {"ByteBuf"}
// Dependencies: {}
# [doc = " A wrapper around `Vec<u8>` to serialize more efficiently."] # [doc = ""] # [doc = " This has a specialized implementation of `RmpWrite`"] # [doc = " It gives `std::convert::Infailable` for errors."] # [doc = " This is because writing to `Vec<T>` can only fail due to allocation."] # [doc = ""] # [doc = " This has the additional benefit of working on `#[no_std]`"] # [doc = ""] # [doc = " See also [serde_bytes::ByteBuf](https://docs.rs/serde_bytes/0.11/serde_bytes/struct.ByteBuf.html)"] # [derive (Debug , Clone , Default , Eq , PartialEq , Hash , Ord , PartialOrd)] pub struct ByteBuf { bytes : Vec < u8 > , }
};
}
