// Generated macro for impl_21 (impl)
macro_rules! Depcrate_future_eitherimpl_21 {
() => {
// Module: crate::future::either
// Provides: {"impl_21"}
// Dependencies: {}
impl < L , R > Future for Either < L , R > where L : Future , R : Future < Output = L :: Output > , { type Output = L :: Output ; # [inline] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . project () { EitherProj :: Left { value } => value . poll (cx) , EitherProj :: Right { value } => value . poll (cx) , } } }
};
}
