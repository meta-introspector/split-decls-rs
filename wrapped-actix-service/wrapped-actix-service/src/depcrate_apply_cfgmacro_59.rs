// Generated macro for macro_59 (macro)
macro_rules! Depcrate_apply_cfgmacro_59 {
() => {
// Module: crate::apply_cfg
// Provides: {"macro_59"}
// Dependencies: {}
pin_project ! { # [project = StateProj] enum State < SF , Fut , S , Req > where SF : ServiceFactory < Req , Config = () >, SF :: InitError : From < SF :: Error >, Fut : Future < Output = Result < S , SF :: InitError >>, S : Service < Req >, { A { # [pin] fut : SF :: Future } , B { svc : SF :: Service } , C { # [pin] fut : Fut } , } }
};
}
