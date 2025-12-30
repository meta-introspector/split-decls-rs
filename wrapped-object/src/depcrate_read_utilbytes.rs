// Generated macro for Bytes (struct)
macro_rules! Depcrate_read_utilBytes {
() => {
// Module: crate::read::util
// Provides: {"Bytes"}
// Dependencies: {}
# [doc = " A newtype for byte slices."] # [doc = ""] # [doc = " It has these important features:"] # [doc = " - no methods that can panic, such as `Index`"] # [doc = " - convenience methods for `Pod` types"] # [doc = " - a useful `Debug` implementation"] # [derive (Default , Clone , Copy , PartialEq , Eq)] pub struct Bytes < 'data > (pub & 'data [u8]) ;
};
}
