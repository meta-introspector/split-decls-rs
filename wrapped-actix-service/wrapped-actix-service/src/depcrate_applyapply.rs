// Generated macro for Apply (struct)
macro_rules! Depcrate_applyApply {
() => {
// Module: crate::apply
// Provides: {"Apply"}
// Dependencies: {}
# [doc = " `Apply` service combinator."] # [doc = ""] # [doc = " The In and Out type params refer to the request and response types for the wrapped service."] pub struct Apply < S , F , Req , In , Res , Err > where S : Service < In , Error = Err > , { service : S , wrap_fn : F , _phantom : PhantomData < fn (Req) -> (In , Res , Err) > , }
};
}
