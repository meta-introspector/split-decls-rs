// Generated macro for impl_2378 (impl)
macro_rules! Depcrate_isa_s390x_inst_argsimpl_2378 {
() => {
// Module: crate::isa::s390x::inst::args
// Provides: {"impl_2378"}
// Dependencies: {}
impl PrettyPrint for Cond { fn pretty_print (& self , _ : u8) -> String { let s = match self . mask { 1 => "o" , 2 => "h" , 3 => "nle" , 4 => "l" , 5 => "nhe" , 6 => "lh" , 7 => "ne" , 8 => "e" , 9 => "nlh" , 10 => "he" , 11 => "nl" , 12 => "le" , 13 => "nh" , 14 => "no" , _ => unreachable ! () , } ; s . to_string () } }
};
}
