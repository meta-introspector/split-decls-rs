// Generated macro for impl_94 (impl)
macro_rules! Depcrate_fn_serviceimpl_94 {
() => {
// Module: crate::fn_service
// Provides: {"impl_94"}
// Dependencies: {}
impl < F , Fut , Req , Res , Err > FnService < F , Fut , Req , Res , Err > where F : FnMut (Req) -> Fut , Fut : Future < Output = Result < Res , Err > > , { pub (crate) fn new (f : F) -> Self { Self { f , _t : PhantomData } } }
};
}
