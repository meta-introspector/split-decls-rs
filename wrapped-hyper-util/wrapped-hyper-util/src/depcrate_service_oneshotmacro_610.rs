// Generated macro for macro_610 (macro)
macro_rules! Depcrate_service_oneshotmacro_610 {
() => {
// Module: crate::service::oneshot
// Provides: {"macro_610"}
// Dependencies: {}
pin_project ! { # [project = OneshotProj] # [derive (Debug)] pub enum Oneshot < S : Service < Req >, Req > { NotReady { svc : S , req : Option < Req >, } , Called { # [pin] fut : S :: Future , } , Done , } }
};
}
