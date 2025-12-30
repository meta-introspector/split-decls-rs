// Generated macro for Octets (struct)
macro_rules! DepcrateOctets {
() => {
// Module: crate
// Provides: {"Octets"}
// Dependencies: {}
# [doc = " A zero-copy immutable byte buffer."] # [doc = ""] # [doc = " `Octets` wraps an in-memory buffer of bytes and provides utility functions"] # [doc = " for manipulating it. The underlying buffer is provided by the user and is"] # [doc = " not copied when creating an `Octets`. Operations are panic-free and will"] # [doc = " avoid indexing the buffer past its end."] # [doc = ""] # [doc = " Additionally, an offset (initially set to the start of the buffer) is"] # [doc = " incremented as bytes are read from / written to the buffer, to allow for"] # [doc = " sequential operations."] # [derive (Debug , PartialEq , Eq)] pub struct Octets < 'a > { buf : & 'a [u8] , off : usize , }
};
}
