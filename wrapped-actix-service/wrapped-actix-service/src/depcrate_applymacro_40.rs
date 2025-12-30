// Generated macro for macro_40 (macro)
macro_rules! Depcrate_applymacro_40 {
() => {
// Module: crate::apply
// Provides: {"macro_40"}
// Dependencies: {}
pin_project ! { pub struct ApplyServiceFactoryResponse < SF , F , Fut , Req , In , Res , Err > where SF : ServiceFactory < In , Error = Err >, F : Fn (Req , & SF :: Service) -> Fut , Fut : Future < Output = Result < Res , Err >>, { # [pin] fut : SF :: Future , wrap_fn : Option < F >, _phantom : PhantomData < fn (Req) -> Res >, } }
};
}
