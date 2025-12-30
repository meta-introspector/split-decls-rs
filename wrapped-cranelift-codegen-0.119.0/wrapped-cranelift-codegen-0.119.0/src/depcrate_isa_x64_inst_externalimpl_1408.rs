// Generated macro for impl_1408 (impl)
macro_rules! Depcrate_isa_x64_inst_externalimpl_1408 {
() => {
// Module: crate::isa::x64::inst::external
// Provides: {"impl_1408"}
// Dependencies: {}
# [doc = " This bridges the gap between codegen and assembler for general purpose register types."] impl asm :: AsReg for Gpr { fn enc (& self) -> u8 { enc_gpr (self) } fn to_string (& self , size : Option < asm :: Size >) -> String { if self . is_real () { asm :: gpr :: enc :: to_string (self . enc () , size . unwrap ()) . into () } else { format ! ("%{:?}" , self . to_reg ()) } } fn new (_ : u8) -> Self { panic ! ("disallow creation of new assembler registers") } }
};
}
