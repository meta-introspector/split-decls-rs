// Generated macro for impl_1748 (impl)
macro_rules! Depcrate_isa_aarch64_inst_argsimpl_1748 {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"impl_1748"}
// Dependencies: {}
impl PrettyPrint for BranchTarget { fn pretty_print (& self , _ : u8) -> String { match self { & BranchTarget :: Label (label) => format ! ("label{:?}" , label . as_u32 ()) , & BranchTarget :: ResolvedOffset (off) => format ! ("{off}") , } } }
};
}
