// Generated macro for unpack_octets_4 (macro)
macro_rules! Depcrate_frameunpack_octets_4 {
() => {
// Module: crate::frame
// Provides: {"unpack_octets_4"}
// Dependencies: {}
# [doc = " A helper macro that unpacks a sequence of 4 bytes found in the buffer with"] # [doc = " the given identifier, starting at the given offset, into the given integer"] # [doc = " type. Obviously, the integer type should be able to support at least 4"] # [doc = " bytes."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " # // We ignore this doctest because the macro is not exported."] # [doc = " let buf: [u8; 4] = [0, 0, 0, 1];"] # [doc = " assert_eq!(1u32, unpack_octets_4!(buf, 0, u32));"] # [doc = " ```"] macro_rules ! unpack_octets_4 { ($ buf : expr , $ offset : expr , $ tip : ty) => { (($ buf [$ offset + 0] as $ tip) << 24) | (($ buf [$ offset + 1] as $ tip) << 16) | (($ buf [$ offset + 2] as $ tip) << 8) | (($ buf [$ offset + 3] as $ tip) << 0) } ; }
};
}
