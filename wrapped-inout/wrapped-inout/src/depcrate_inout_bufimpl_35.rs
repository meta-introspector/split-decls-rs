// Generated macro for impl_35 (impl)
macro_rules! Depcrate_inout_bufimpl_35 {
() => {
// Module: crate::inout_buf
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a , T > InOutBuf < 'a , 'a , T > { # [doc = " Create `InOutBuf` from a single mutable reference."] # [inline (always)] pub fn from_mut (val : & 'a mut T) -> InOutBuf < 'a , 'a , T > { let p = val as * mut T ; Self { in_ptr : p , out_ptr : p , len : 1 , _pd : PhantomData , } } }
};
}
