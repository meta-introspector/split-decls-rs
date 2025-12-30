// Generated macro for impl_130 (impl)
macro_rules! Depcrate_mapimpl_130 {
() => {
// Module: crate::map
// Provides: {"impl_130"}
// Dependencies: {}
impl < A , F , Req , Res > MapServiceFactory < A , F , Req , Res > { # [doc = " Create new `Map` new service instance"] pub (crate) fn new (a : A , f : F) -> Self where A : ServiceFactory < Req > , F : FnMut (A :: Response) -> Res , { Self { a , f , r : PhantomData , } } }
};
}
