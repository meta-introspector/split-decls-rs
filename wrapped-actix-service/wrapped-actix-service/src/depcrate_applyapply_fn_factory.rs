// Generated macro for apply_fn_factory (function)
macro_rules! Depcrate_applyapply_fn_factory {
() => {
// Module: crate::apply
// Provides: {"apply_fn_factory"}
// Dependencies: {}
# [doc = " Service factory that produces `apply_fn` service."] # [doc = ""] # [doc = " The In and Out type params refer to the request and response types for the wrapped service."] pub fn apply_fn_factory < I , SF , F , Fut , Req , In , Res , Err > (service : I , f : F ,) -> ApplyFactory < SF , F , Req , In , Res , Err > where I : IntoServiceFactory < SF , In > , SF : ServiceFactory < In , Error = Err > , F : Fn (Req , & SF :: Service) -> Fut + Clone , Fut : Future < Output = Result < Res , Err > > , { ApplyFactory :: new (service . into_factory () , f) }
};
}
