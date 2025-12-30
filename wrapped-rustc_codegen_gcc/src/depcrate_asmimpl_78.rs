// Generated macro for impl_78 (impl)
macro_rules! Depcrate_asmimpl_78 {
() => {
// Module: crate::asm
// Provides: {"impl_78"}
// Dependencies: {}
impl AsmOutOperand < '_ , '_ , '_ > { fn to_constraint (& self) -> String { let mut res = String :: with_capacity (self . constraint . len () + self . late as usize + 1) ; let sign = if self . readwrite { '+' } else { '=' } ; res . push (sign) ; if ! self . late { res . push ('&') ; } res . push_str (self . constraint) ; res } }
};
}
