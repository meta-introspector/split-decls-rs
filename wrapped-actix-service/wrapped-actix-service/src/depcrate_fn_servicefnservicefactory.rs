// Generated macro for FnServiceFactory (struct)
macro_rules! Depcrate_fn_serviceFnServiceFactory {
() => {
// Module: crate::fn_service
// Provides: {"FnServiceFactory"}
// Dependencies: {}
pub struct FnServiceFactory < F , Fut , Req , Res , Err , Cfg > where F : Fn (Req) -> Fut , Fut : Future < Output = Result < Res , Err > > , { f : F , _t : PhantomData < fn (Req , Cfg) > , }
};
}
