// Generated macro for OctetsMut (struct)
macro_rules! DepcrateOctetsMut {
() => {
// Module: crate
// Provides: {"OctetsMut"}
// Dependencies: {}
# [doc = " A zero-copy mutable byte buffer."] # [doc = ""] # [doc = " Like `Octets` but mutable."] # [derive (Debug , PartialEq , Eq)] pub struct OctetsMut < 'a > { buf : & 'a mut [u8] , off : usize , }
};
}
