// Generated macro for impl_216 (impl)
macro_rules! Depcrate_thenimpl_216 {
() => {
// Module: crate::then
// Provides: {"impl_216"}
// Dependencies: {}
impl < A , B , Req > ThenServiceFactory < A , B , Req > where A : ServiceFactory < Req > , A :: Config : Clone , B : ServiceFactory < Result < A :: Response , A :: Error > , Config = A :: Config , Error = A :: Error , InitError = A :: InitError , > , { # [doc = " Create new `AndThen` combinator"] pub (crate) fn new (a : A , b : B) -> Self { Self (Rc :: new ((a , b)) , PhantomData) } }
};
}
