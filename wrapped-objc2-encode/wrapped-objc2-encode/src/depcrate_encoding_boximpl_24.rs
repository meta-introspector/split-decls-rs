// Generated macro for impl_24 (impl)
macro_rules! Depcrate_encoding_boximpl_24 {
() => {
// Module: crate::encoding_box
// Provides: {"impl_24"}
// Dependencies: {}
# [doc = " Same formatting as [`Encoding`]'s `Display` implementation."] impl fmt :: Display for EncodingBox { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { Helper :: from_box (self) . fmt (f , NestingLevel :: new ()) } }
};
}
