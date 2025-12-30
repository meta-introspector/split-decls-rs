// Generated macro for check_unop (function)
macro_rules! Depcrate_machinst_pcccheck_unop {
() => {
// Module: crate::machinst::pcc
// Provides: {"check_unop"}
// Dependencies: {}
pub (crate) fn check_unop < I : VCodeInst , F : FnOnce (& Fact) -> PccResult < Option < Fact > > > (ctx : & FactContext , vcode : & mut VCode < I > , reg_width : u16 , out : Writable < Reg > , ra : Reg , f : F ,) -> PccResult < () > { check_output (ctx , vcode , out , & [ra] , | vcode | { let ra = get_fact_or_default (vcode , ra , reg_width) ; f (& ra) }) }
};
}
