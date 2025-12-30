// Generated macro for impl_104 (impl)
macro_rules! Depcrate_eitherimpl_104 {
() => {
// Module: crate::either
// Provides: {"impl_104"}
// Dependencies: {}
impl < L , R > Either < L , R > { # [doc = " This function is part of the generated code from `pin-project-lite`,"] # [doc = " for a more in depth explanation and the rest of the generated code refer"] # [doc = " to the [`proj`] module."] pub (crate) fn project (self : Pin < & mut Self >) -> EitherProj < '_ , L , R > { unsafe { match self . get_unchecked_mut () { Self :: Left (left) => EitherProj :: Left (Pin :: new_unchecked (left)) , Self :: Right (right) => EitherProj :: Right (Pin :: new_unchecked (right)) , } } } }
};
}
