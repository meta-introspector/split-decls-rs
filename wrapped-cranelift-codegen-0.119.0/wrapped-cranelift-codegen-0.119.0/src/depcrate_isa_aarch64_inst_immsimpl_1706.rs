// Generated macro for impl_1706 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1706 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1706"}
// Dependencies: {}
impl PrettyPrint for NZCV { fn pretty_print (& self , _ : u8) -> String { let fmt = | c : char , v | if v { c . to_ascii_uppercase () } else { c } ; format ! ("#{}{}{}{}" , fmt ('n' , self . n) , fmt ('z' , self . z) , fmt ('c' , self . c) , fmt ('v' , self . v)) } }
};
}
