// Generated macro for ByteArray (type)
macro_rules! Depcrate_arrayByteArray {
() => {
// Module: crate::array
// Provides: {"ByteArray"}
// Dependencies: {}
# [doc = " Alias for a byte array whose size is defined by [`ArrayEncoding::ByteSize`]."] pub type ByteArray < T > = Array < u8 , < T as ArrayEncoding > :: ByteSize > ;
};
}
