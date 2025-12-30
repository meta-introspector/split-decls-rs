// Generated macro for impl_1409 (impl)
macro_rules! Depcrate_isa_x64_inst_externalimpl_1409 {
() => {
// Module: crate::isa::x64::inst::external
// Provides: {"impl_1409"}
// Dependencies: {}
# [doc = " This bridges the gap between codegen and assembler for xmm register types."] impl asm :: AsReg for Xmm { fn enc (& self) -> u8 { enc_xmm (self) } fn to_string (& self , size : Option < asm :: Size >) -> String { assert ! (size . is_none () , "XMM registers do not have size variants") ; if self . is_real () { asm :: xmm :: enc :: to_string (self . enc ()) . into () } else { format ! ("%{:?}" , self . to_reg ()) } } fn new (_ : u8) -> Self { panic ! ("disallow creation of new assembler registers") } }
};
}
