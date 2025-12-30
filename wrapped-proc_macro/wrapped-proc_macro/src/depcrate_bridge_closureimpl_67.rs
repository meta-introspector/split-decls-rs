// Generated macro for impl_67 (impl)
macro_rules! Depcrate_bridge_closureimpl_67 {
() => {
// Module: crate::bridge::closure
// Provides: {"impl_67"}
// Dependencies: {}
impl < 'a , A , R , F : FnMut (A) -> R > From < & 'a mut F > for Closure < 'a , A , R > { fn from (f : & 'a mut F) -> Self { unsafe extern "C" fn call < A , R , F : FnMut (A) -> R > (env : * mut Env , arg : A) -> R { unsafe { (* (env as * mut _ as * mut F)) (arg) } } Closure { call : call :: < A , R , F > , env : f as * mut _ as * mut Env , _marker : PhantomData } } }
};
}
