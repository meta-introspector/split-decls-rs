// Generated macro for impl_18 (impl)
macro_rules! Depcrate_and_thenimpl_18 {
() => {
// Module: crate::and_then
// Provides: {"impl_18"}
// Dependencies: {}
impl < A , B , Req > AndThenServiceFactory < A , B , Req > where A : ServiceFactory < Req > , A :: Config : Clone , B : ServiceFactory < A :: Response , Config = A :: Config , Error = A :: Error , InitError = A :: InitError > , { # [doc = " Create new `AndThenFactory` combinator"] pub (crate) fn new (a : A , b : B) -> Self { Self { inner : Rc :: new ((a , b)) , _phantom : PhantomData , } } }
};
}
