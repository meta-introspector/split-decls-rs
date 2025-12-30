// Generated macro for impl_64 (impl)
macro_rules! Depcrate_body_lengthimpl_64 {
() => {
// Module: crate::body::length
// Provides: {"impl_64"}
// Dependencies: {}
impl fmt :: Display for DecodedLength { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { DecodedLength :: CLOSE_DELIMITED => f . write_str ("close-delimited") , DecodedLength :: CHUNKED => f . write_str ("chunked encoding") , DecodedLength :: ZERO => f . write_str ("empty") , DecodedLength (n) => write ! (f , "content-length ({} bytes)" , n) , } } }
};
}
