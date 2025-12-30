// Generated macro for impl_209 (impl)
macro_rules! Depcrate_thenimpl_209 {
() => {
// Module: crate::then
// Provides: {"impl_209"}
// Dependencies: {}
impl < A , B , Req > ThenService < A , B , Req > { # [doc = " Create new `.then()` combinator"] pub (crate) fn new (a : A , b : B) -> ThenService < A , B , Req > where A : Service < Req > , B : Service < Result < A :: Response , A :: Error > , Error = A :: Error > , { Self (Rc :: new ((a , b)) , PhantomData) } }
};
}
