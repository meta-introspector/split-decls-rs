// Generated macro for impl_188 (impl)
macro_rules! Depcrate_pipelineimpl_188 {
() => {
// Module: crate::pipeline
// Provides: {"impl_188"}
// Dependencies: {}
impl < S : Service < Req > , Req > Service < Req > for Pipeline < S , Req > { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn poll_ready (& self , ctx : & mut Context < '_ >) -> Poll < Result < () , S :: Error > > { self . service . poll_ready (ctx) } # [inline] fn call (& self , req : Req) -> Self :: Future { self . service . call (req) } }
};
}
