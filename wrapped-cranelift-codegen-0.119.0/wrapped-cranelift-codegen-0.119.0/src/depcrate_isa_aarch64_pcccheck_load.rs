// Generated macro for check_load (function)
macro_rules! Depcrate_isa_aarch64_pcccheck_load {
() => {
// Module: crate::isa::aarch64::pcc
// Provides: {"check_load"}
// Dependencies: {}
fn check_load (ctx : & FactContext , rd : Option < Reg > , flags : MemFlags , addr : & AMode , vcode : & VCode < Inst > , ty : Type ,) -> PccResult < () > { let result_fact = rd . and_then (| rd | vcode . vreg_fact (rd . into ())) ; let bits = u16 :: try_from (ty . bits ()) . unwrap () ; check_addr (ctx , flags , addr , vcode , ty , LoadOrStore :: Load { result_fact , from_bits : bits , to_bits : bits , } ,) }
};
}
