// Generated macro for check_store (function)
macro_rules! Depcrate_isa_aarch64_pcccheck_store {
() => {
// Module: crate::isa::aarch64::pcc
// Provides: {"check_store"}
// Dependencies: {}
fn check_store (ctx : & FactContext , rd : Option < Reg > , flags : MemFlags , addr : & AMode , vcode : & VCode < Inst > , ty : Type ,) -> PccResult < () > { let stored_fact = rd . and_then (| rd | vcode . vreg_fact (rd . into ())) ; check_addr (ctx , flags , addr , vcode , ty , LoadOrStore :: Store { stored_fact } ,) }
};
}
