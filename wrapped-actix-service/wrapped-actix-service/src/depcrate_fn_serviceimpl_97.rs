// Generated macro for impl_97 (impl)
macro_rules! Depcrate_fn_serviceimpl_97 {
() => {
// Module: crate::fn_service
// Provides: {"impl_97"}
// Dependencies: {}
impl < F , Fut , Req , Res , Err > IntoService < FnService < F , Fut , Req , Res , Err > , Req > for F where F : Fn (Req) -> Fut , Fut : Future < Output = Result < Res , Err > > , { fn into_service (self) -> FnService < F , Fut , Req , Res , Err > { FnService :: new (self) } }
};
}
