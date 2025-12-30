// Generated macro for impl_23 (impl)
macro_rules! Depcrate_inoutimpl_23 {
() => {
// Module: crate::inout
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'a , T > From < & 'a mut T > for InOut < 'a , 'a , T > { # [inline (always)] fn from (val : & 'a mut T) -> Self { let p = val as * mut T ; Self { in_ptr : p , out_ptr : p , _pd : PhantomData , } } }
};
}
