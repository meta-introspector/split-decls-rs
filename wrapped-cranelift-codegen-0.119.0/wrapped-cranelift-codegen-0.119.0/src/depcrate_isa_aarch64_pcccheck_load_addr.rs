// Generated macro for check_load_addr (function)
macro_rules! Depcrate_isa_aarch64_pcccheck_load_addr {
() => {
// Module: crate::isa::aarch64::pcc
// Provides: {"check_load_addr"}
// Dependencies: {}
fn check_load_addr (ctx : & FactContext , flags : MemFlags , reg : Reg , vcode : & VCode < Inst > , ty : Type ,) -> PccResult < () > { if ! flags . checked () { return Ok (()) ; } let fact = get_fact_or_default (vcode , reg , 64) ; let _output_fact = ctx . load (& fact , ty) ? ; Ok (()) }
};
}
