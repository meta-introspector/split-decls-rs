// Generated macro for impl_25 (impl)
macro_rules! Depcrate_combinators_box_bodyimpl_25 {
() => {
// Module: crate::combinators::box_body
// Provides: {"impl_25"}
// Dependencies: {}
impl < D , E > UnsyncBoxBody < D , E > { # [doc = " Create a new `UnsyncBoxBody`."] pub fn new < B > (body : B) -> Self where B : Body < Data = D , Error = E > + Send + 'static , D : Buf , { Self { inner : Box :: pin (body) , } } }
};
}
