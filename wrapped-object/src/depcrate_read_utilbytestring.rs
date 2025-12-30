// Generated macro for ByteString (struct)
macro_rules! Depcrate_read_utilByteString {
() => {
// Module: crate::read::util
// Provides: {"ByteString"}
// Dependencies: {}
# [doc = " A newtype for byte strings."] # [doc = ""] # [doc = " For byte slices that are strings of an unknown encoding."] # [doc = ""] # [doc = " Provides a `Debug` implementation that interprets the bytes as UTF-8."] # [derive (Default , Clone , Copy , PartialEq , Eq)] pub (crate) struct ByteString < 'data > (pub & 'data [u8]) ;
};
}
