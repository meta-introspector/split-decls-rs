// Generated macro for check_binop (function)
macro_rules! Depcrate_machinst_pcccheck_binop {
() => {
// Module: crate::machinst::pcc
// Provides: {"check_binop"}
// Dependencies: {}
pub (crate) fn check_binop < I : VCodeInst , F : FnOnce (& Fact , & Fact) -> PccResult < Option < Fact > > > (ctx : & FactContext , vcode : & mut VCode < I > , reg_width : u16 , out : Writable < Reg > , ra : Reg , rb : Reg , f : F ,) -> PccResult < () > { check_output (ctx , vcode , out , & [ra , rb] , | vcode | { let ra = get_fact_or_default (vcode , ra , reg_width) ; let rb = get_fact_or_default (vcode , rb , reg_width) ; f (& ra , & rb) }) }
};
}
