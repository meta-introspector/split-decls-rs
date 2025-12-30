// Generated macro for impl_502 (impl)
macro_rules! Depcrate_future_eitherimpl_502 {
() => {
// Module: crate::future::either
// Provides: {"impl_502"}
// Dependencies: {}
impl < A , B , T > Either < (A , T) , (B , T) > { # [doc = " Factor out a homogeneous type from an either of pairs."] # [doc = ""] # [doc = " Here, the homogeneous type is the second element of the pairs."] pub fn factor_second (self) -> (Either < A , B > , T) { match self { Self :: Left ((a , x)) => (Either :: Left (a) , x) , Self :: Right ((b , x)) => (Either :: Right (b) , x) , } } }
};
}
