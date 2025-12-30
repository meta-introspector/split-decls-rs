// Generated macro for impl_504 (impl)
macro_rules! Depcrate_future_eitherimpl_504 {
() => {
// Module: crate::future::either
// Provides: {"impl_504"}
// Dependencies: {}
impl < A , B > Future for Either < A , B > where A : Future , B : Future < Output = A :: Output > , { type Output = A :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . as_pin_mut () { Either :: Left (x) => x . poll (cx) , Either :: Right (x) => x . poll (cx) , } } }
};
}
