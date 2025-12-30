// Generated macro for check_load (function)
macro_rules! Depcrate_isa_x64_pcccheck_load {
() => {
// Module: crate::isa::x64::pcc
// Provides: {"check_load"}
// Dependencies: {}
fn check_load (ctx : & FactContext , dst : Option < Writable < Reg > > , src : & SyntheticAmode , vcode : & VCode < Inst > , ty : Type , to_bits : u16 ,) -> PccResult < Option < Fact > > { let result_fact = dst . and_then (| dst | vcode . vreg_fact (dst . to_reg () . into ())) ; let from_bits = u16 :: try_from (ty . bits ()) . unwrap () ; check_mem (ctx , src , vcode , ty , LoadOrStore :: Load { result_fact , from_bits , to_bits , } ,) }
};
}
