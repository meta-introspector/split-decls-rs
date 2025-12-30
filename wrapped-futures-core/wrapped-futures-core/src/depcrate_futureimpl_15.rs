// Generated macro for impl_15 (impl)
macro_rules! Depcrate_futureimpl_15 {
() => {
// Module: crate::future
// Provides: {"impl_15"}
// Dependencies: {}
impl < F , T , E > TryFuture for F where F : ? Sized + Future < Output = Result < T , E > > , { type Ok = T ; type Error = E ; # [inline] fn try_poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . poll (cx) } }
};
}
