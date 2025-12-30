// Generated macro for impl_1743 (impl)
macro_rules! Depcrate_isa_aarch64_inst_argsimpl_1743 {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"impl_1743"}
// Dependencies: {}
impl PrettyPrint for MemLabel { fn pretty_print (& self , _ : u8) -> String { match self { MemLabel :: PCRel (off) => format ! ("pc+{off}") , MemLabel :: Mach (off) => format ! ("label({})" , off . as_u32 ()) , } } }
};
}
