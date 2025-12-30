// Generated macro for ensure_no_fact (function)
macro_rules! Depcrate_isa_x64_pccensure_no_fact {
() => {
// Module: crate::isa::x64::pcc
// Provides: {"ensure_no_fact"}
// Dependencies: {}
fn ensure_no_fact (vcode : & VCode < Inst > , reg : Reg) -> PccResult < () > { if vcode . vreg_fact (reg . into ()) . is_some () { Err (PccError :: UnsupportedFact) } else { Ok (()) } }
};
}
