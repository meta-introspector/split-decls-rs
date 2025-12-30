// Generated macro for impl_11 (impl)
macro_rules! Depcrate_and_thenimpl_11 {
() => {
// Module: crate::and_then
// Provides: {"impl_11"}
// Dependencies: {}
impl < A , B , Req > AndThenService < A , B , Req > { # [doc = " Create new `AndThen` combinator"] pub (crate) fn new (a : A , b : B) -> Self where A : Service < Req > , B : Service < A :: Response , Error = A :: Error > , { Self (Rc :: new ((a , b)) , PhantomData) } }
};
}
