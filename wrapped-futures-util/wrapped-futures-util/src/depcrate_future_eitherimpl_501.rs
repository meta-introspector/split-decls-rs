// Generated macro for impl_501 (impl)
macro_rules! Depcrate_future_eitherimpl_501 {
() => {
// Module: crate::future::either
// Provides: {"impl_501"}
// Dependencies: {}
impl < A , B , T > Either < (T , A) , (T , B) > { # [doc = " Factor out a homogeneous type from an either of pairs."] # [doc = ""] # [doc = " Here, the homogeneous type is the first element of the pairs."] pub fn factor_first (self) -> (T , Either < A , B >) { match self { Self :: Left ((x , a)) => (x , Either :: Left (a)) , Self :: Right ((x , b)) => (x , Either :: Right (b)) , } } }
};
}
