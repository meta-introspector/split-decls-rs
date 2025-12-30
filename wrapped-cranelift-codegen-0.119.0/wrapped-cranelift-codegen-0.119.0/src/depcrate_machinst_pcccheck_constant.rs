// Generated macro for check_constant (function)
macro_rules! Depcrate_machinst_pcccheck_constant {
() => {
// Module: crate::machinst::pcc
// Provides: {"check_constant"}
// Dependencies: {}
pub (crate) fn check_constant < I : VCodeInst > (ctx : & FactContext , vcode : & mut VCode < I > , out : Writable < Reg > , bit_width : u16 , value : u64 ,) -> PccResult < () > { let result = Fact :: constant (bit_width , value) ; if let Some (fact) = vcode . vreg_fact (out . to_reg () . into ()) { check_subsumes (ctx , & result , fact) } else { trace ! ("setting vreg {:?} to {:?}" , out , result) ; vcode . set_vreg_fact (out . to_reg () . into () , result) ; Ok (()) } }
};
}
