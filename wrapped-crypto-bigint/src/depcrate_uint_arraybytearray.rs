// Generated macro for ByteArray (type)
macro_rules! Depcrate_uint_arrayByteArray {
() => {
// Module: crate::uint::array
// Provides: {"ByteArray"}
// Dependencies: {}
# [doc = " Alias for a byte array whose size is defined by [`ArrayEncoding::ByteSize`]."] pub type ByteArray < T > = Array < u8 , < T as ArrayEncoding > :: ByteSize > ;
};
}
