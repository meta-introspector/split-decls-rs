// Generated macro for impl_123 (impl)
macro_rules! Depcrate_mapimpl_123 {
() => {
// Module: crate::map
// Provides: {"impl_123"}
// Dependencies: {}
impl < A , F , Req , Res > Map < A , F , Req , Res > { # [doc = " Create new `Map` combinator"] pub (crate) fn new (service : A , f : F) -> Self where A : Service < Req > , F : FnMut (A :: Response) -> Res , { Self { service , f , _t : PhantomData , } } }
};
}
