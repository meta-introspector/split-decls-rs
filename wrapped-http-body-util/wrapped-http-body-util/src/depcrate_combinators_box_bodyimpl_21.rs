// Generated macro for impl_21 (impl)
macro_rules! Depcrate_combinators_box_bodyimpl_21 {
() => {
// Module: crate::combinators::box_body
// Provides: {"impl_21"}
// Dependencies: {}
impl < D , E > BoxBody < D , E > { # [doc = " Create a new `BoxBody`."] pub fn new < B > (body : B) -> Self where B : Body < Data = D , Error = E > + Send + Sync + 'static , D : Buf , { Self { inner : Box :: pin (body) , } } }
};
}
