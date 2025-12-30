// Generated macro for impl_123 (impl)
macro_rules! Depcrate_pemimpl_123 {
() => {
// Module: crate::pem
// Provides: {"impl_123"}
// Dependencies: {}
# [cfg (feature = "std")] impl < R : io :: BufRead , T : PemObject > ReadIter < R , T > { # [doc = " Create a new iterator."] pub fn new (rd : R) -> Self { Self { rd , _ty : PhantomData , line : Vec :: with_capacity (80) , b64_buf : Vec :: with_capacity (1024) , } } }
};
}
