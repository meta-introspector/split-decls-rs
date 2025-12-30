// Generated macro for slice_to_u8_mut (function)
macro_rules! Depcrate_ioslice_to_u8_mut {
() => {
// Module: crate::io
// Provides: {"slice_to_u8_mut"}
// Dependencies: {}
# [doc = " Convert a slice of T (where T is plain old data) to its mutable binary"] # [doc = " representation."] # [doc = ""] # [doc = " This function is wildly unsafe because it permits arbitrary modification of"] # [doc = " the binary representation of any `Copy` type. Use with care. It's intended"] # [doc = " to be called only where `T` is a numeric type."] unsafe fn slice_to_u8_mut < T : Copy > (slice : & mut [T]) -> & mut [u8] { use std :: mem :: size_of ; let len = size_of :: < T > () * slice . len () ; slice :: from_raw_parts_mut (slice . as_mut_ptr () as * mut u8 , len) }
};
}
