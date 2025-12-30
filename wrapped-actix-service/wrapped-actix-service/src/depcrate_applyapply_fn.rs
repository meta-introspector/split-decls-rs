// Generated macro for apply_fn (function)
macro_rules! Depcrate_applyapply_fn {
() => {
// Module: crate::apply
// Provides: {"apply_fn"}
// Dependencies: {}
# [doc = " Apply transform function to a service."] # [doc = ""] # [doc = " The In and Out type params refer to the request and response types for the wrapped service."] pub fn apply_fn < I , S , F , Fut , Req , In , Res , Err > (service : I , wrap_fn : F ,) -> Apply < S , F , Req , In , Res , Err > where I : IntoService < S , In > , S : Service < In , Error = Err > , F : Fn (Req , & S) -> Fut , Fut : Future < Output = Result < Res , Err > > , { Apply :: new (service . into_service () , wrap_fn) }
};
}
