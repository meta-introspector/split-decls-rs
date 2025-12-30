// Generated macro for FnService (struct)
macro_rules! Depcrate_fn_serviceFnService {
() => {
// Module: crate::fn_service
// Provides: {"FnService"}
// Dependencies: {}
pub struct FnService < F , Fut , Req , Res , Err > where F : FnMut (Req) -> Fut , Fut : Future < Output = Result < Res , Err > > , { f : F , _t : PhantomData < fn (Req) > , }
};
}
