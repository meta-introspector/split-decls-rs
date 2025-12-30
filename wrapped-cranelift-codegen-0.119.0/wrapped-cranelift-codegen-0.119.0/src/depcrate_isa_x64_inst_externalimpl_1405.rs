// Generated macro for impl_1405 (impl)
macro_rules! Depcrate_isa_x64_inst_externalimpl_1405 {
() => {
// Module: crate::isa::x64::inst::external
// Provides: {"impl_1405"}
// Dependencies: {}
impl asm :: AsReg for PairedGpr { fn enc (& self) -> u8 { let PairedGpr { read , write } = self ; let read = enc_gpr (read) ; let write = enc_gpr (& write . to_reg ()) ; assert_eq ! (read , write) ; write } fn to_string (& self , size : Option < asm :: Size >) -> String { if self . read . is_real () { asm :: gpr :: enc :: to_string (self . enc () , size . unwrap ()) . into () } else { let read = self . read . to_reg () ; let write = self . write . to_reg () . to_reg () ; format ! ("(%{write:?} <- %{read:?})") } } fn new (_ : u8) -> Self { panic ! ("disallow creation of new assembler registers") } }
};
}
