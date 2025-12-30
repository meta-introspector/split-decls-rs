// Generated macro for impl_186 (impl)
macro_rules! Depcrate_tests_register_buf_ringimpl_186 {
() => {
// Module: crate::tests::register_buf_ring
// Provides: {"impl_186"}
// Dependencies: {}
impl fmt :: Debug for GBuf { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("GBuf") . field ("bgid" , & self . bufgroup . rc . bgid ()) . field ("bid" , & self . bid) . field ("len" , & self . len) . field ("cap" , & self . bufgroup . rc . buf_capacity ()) . finish () } }
};
}
