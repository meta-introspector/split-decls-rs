// Generated macro for impl_63 (impl)
macro_rules! Depcrate_body_lengthimpl_63 {
() => {
// Module: crate::body::length
// Provides: {"impl_63"}
// Dependencies: {}
impl fmt :: Debug for DecodedLength { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { DecodedLength :: CLOSE_DELIMITED => f . write_str ("CLOSE_DELIMITED") , DecodedLength :: CHUNKED => f . write_str ("CHUNKED") , DecodedLength (n) => f . debug_tuple ("DecodedLength") . field (& n) . finish () , } } }
};
}
