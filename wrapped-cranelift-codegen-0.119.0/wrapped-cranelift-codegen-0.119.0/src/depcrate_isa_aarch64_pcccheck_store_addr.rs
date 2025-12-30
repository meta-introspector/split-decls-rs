// Generated macro for check_store_addr (function)
macro_rules! Depcrate_isa_aarch64_pcccheck_store_addr {
() => {
// Module: crate::isa::aarch64::pcc
// Provides: {"check_store_addr"}
// Dependencies: {}
fn check_store_addr (ctx : & FactContext , flags : MemFlags , reg : Reg , vcode : & VCode < Inst > , ty : Type ,) -> PccResult < () > { if ! flags . checked () { return Ok (()) ; } let fact = get_fact_or_default (vcode , reg , 64) ; let _output_fact = ctx . store (& fact , ty , None) ? ; Ok (()) }
};
}
