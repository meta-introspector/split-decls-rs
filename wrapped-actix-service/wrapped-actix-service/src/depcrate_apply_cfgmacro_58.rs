// Generated macro for macro_58 (macro)
macro_rules! Depcrate_apply_cfgmacro_58 {
() => {
// Module: crate::apply_cfg
// Provides: {"macro_58"}
// Dependencies: {}
pin_project ! { struct ApplyConfigServiceFactoryResponse < SF , Req , F , Cfg , Fut , S > where SF : ServiceFactory < Req , Config = () >, SF :: InitError : From < SF :: Error >, F : Fn (Cfg , & SF :: Service) -> Fut , Fut : Future < Output = Result < S , SF :: InitError >>, S : Service < Req >, { cfg : Option < Cfg >, store : Rc < (SF , F) >, # [pin] state : State < SF , Fut , S , Req >, } }
};
}
