// Generated macro for check_store (function)
macro_rules! Depcrate_isa_x64_pcccheck_store {
() => {
// Module: crate::isa::x64::pcc
// Provides: {"check_store"}
// Dependencies: {}
fn check_store (ctx : & FactContext , data : Option < Reg > , dst : & SyntheticAmode , vcode : & VCode < Inst > , ty : Type ,) -> PccResult < () > { let stored_fact = data . and_then (| data | vcode . vreg_fact (data . into ())) ; check_mem (ctx , dst , vcode , ty , LoadOrStore :: Store { stored_fact }) . map (| _ | ()) }
};
}
