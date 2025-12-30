// Generated macro for impl_19 (impl)
macro_rules! Depcrate_future_eitherimpl_19 {
() => {
// Module: crate::future::either
// Provides: {"impl_19"}
// Dependencies: {}
impl < L , R > Either < L , R > { # [doc = " Creates new `Either` using left variant."] # [inline] pub fn left (value : L) -> Either < L , R > { Either :: Left { value } } # [doc = " Creates new `Either` using right variant."] # [inline] pub fn right (value : R) -> Either < L , R > { Either :: Right { value } } }
};
}
