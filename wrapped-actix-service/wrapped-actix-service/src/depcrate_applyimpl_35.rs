// Generated macro for impl_35 (impl)
macro_rules! Depcrate_applyimpl_35 {
() => {
// Module: crate::apply
// Provides: {"impl_35"}
// Dependencies: {}
impl < S , F , Fut , Req , In , Res , Err > Service < Req > for Apply < S , F , Req , In , Res , Err > where S : Service < In , Error = Err > , F : Fn (Req , & S) -> Fut , Fut : Future < Output = Result < Res , Err > > , { type Response = Res ; type Error = Err ; type Future = Fut ; crate :: forward_ready ! (service) ; fn call (& self , req : Req) -> Self :: Future { (self . wrap_fn) (req , & self . service) } }
};
}
