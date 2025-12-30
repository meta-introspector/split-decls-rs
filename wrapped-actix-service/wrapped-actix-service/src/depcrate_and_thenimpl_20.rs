// Generated macro for impl_20 (impl)
macro_rules! Depcrate_and_thenimpl_20 {
() => {
// Module: crate::and_then
// Provides: {"impl_20"}
// Dependencies: {}
impl < A , B , Req > Clone for AndThenServiceFactory < A , B , Req > where A : ServiceFactory < Req > , A :: Config : Clone , B : ServiceFactory < A :: Response , Config = A :: Config , Error = A :: Error , InitError = A :: InitError > , { fn clone (& self) -> Self { Self { inner : self . inner . clone () , _phantom : PhantomData , } } }
};
}
