// Generated macro for impl_422 (impl)
macro_rules! Depcrate_future_futures_extimpl_422 {
() => {
// Module: crate::future::futures_ext
// Provides: {"impl_422"}
// Dependencies: {}
impl < F1 > FutureExt for F1 where F1 : Future , { fn join < F2 > (self , other : F2) -> Join2 < Self , F2 :: IntoFuture > where Self : Future + Sized , F2 : IntoFuture , { Join :: join ((self , other)) } fn race < T , S2 > (self , other : S2) -> Race2 < T , Self , S2 :: IntoFuture > where Self : Future < Output = T > + Sized , S2 : IntoFuture < Output = T > , { Race :: race ((self , other)) } }
};
}
