// Generated macro for check_output (function)
macro_rules! Depcrate_machinst_pcccheck_output {
() => {
// Module: crate::machinst::pcc
// Provides: {"check_output"}
// Dependencies: {}
pub (crate) fn check_output < I : VCodeInst , F : FnOnce (& VCode < I >) -> PccResult < Option < Fact > > > (ctx : & FactContext , vcode : & mut VCode < I > , out : Writable < Reg > , ins : & [Reg] , f : F ,) -> PccResult < () > { if let Some (fact) = vcode . vreg_fact (out . to_reg () . into ()) { let result = f (vcode) ? ; check_subsumes_optionals (ctx , result . as_ref () , Some (fact)) } else if ins . iter () . any (| r | { vcode . vreg_fact (r . into ()) . map (| fact | fact . propagates ()) . unwrap_or (false) }) { if let Ok (Some (fact)) = f (vcode) { trace ! ("setting vreg {:?} to {:?}" , out , fact) ; vcode . set_vreg_fact (out . to_reg () . into () , fact) ; } Ok (()) } else { Ok (()) } }
};
}
