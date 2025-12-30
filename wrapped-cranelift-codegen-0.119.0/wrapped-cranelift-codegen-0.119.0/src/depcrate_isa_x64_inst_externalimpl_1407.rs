// Generated macro for impl_1407 (impl)
macro_rules! Depcrate_isa_x64_inst_externalimpl_1407 {
() => {
// Module: crate::isa::x64::inst::external
// Provides: {"impl_1407"}
// Dependencies: {}
impl asm :: AsReg for PairedXmm { fn enc (& self) -> u8 { let PairedXmm { read , write } = self ; let read = enc_xmm (read) ; let write = enc_xmm (& write . to_reg ()) ; assert_eq ! (read , write) ; write } fn to_string (& self , size : Option < asm :: Size >) -> String { assert ! (size . is_none () , "XMM registers do not have size variants") ; if self . read . is_real () { asm :: xmm :: enc :: to_string (self . enc ()) . into () } else { let read = self . read . to_reg () ; let write = self . write . to_reg () . to_reg () ; format ! ("(%{write:?} <- %{read:?})") } } fn new (_ : u8) -> Self { panic ! ("disallow creation of new assembler registers") } }
};
}
