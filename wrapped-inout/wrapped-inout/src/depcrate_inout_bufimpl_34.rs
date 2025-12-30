// Generated macro for impl_34 (impl)
macro_rules! Depcrate_inout_bufimpl_34 {
() => {
// Module: crate::inout_buf
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'a , T > From < & 'a mut [T] > for InOutBuf < 'a , 'a , T > { # [inline (always)] fn from (buf : & 'a mut [T]) -> Self { let p = buf . as_mut_ptr () ; Self { in_ptr : p , out_ptr : p , len : buf . len () , _pd : PhantomData , } } }
};
}
