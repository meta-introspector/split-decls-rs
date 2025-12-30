// Generated macro for ExtMode (enum)
macro_rules! Depcrate_isa_x64_inst_argsExtMode {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"ExtMode"}
// Dependencies: {}
# [doc = " These indicate ways of extending (widening) a value, using the Intel"] # [doc = " naming: B(yte) = u8, W(ord) = u16, L(ong)word = u32, Q(uad)word = u64"] # [derive (Clone , PartialEq)] pub enum ExtMode { # [doc = " Byte -> Longword."] BL , # [doc = " Byte -> Quadword."] BQ , # [doc = " Word -> Longword."] WL , # [doc = " Word -> Quadword."] WQ , # [doc = " Longword -> Quadword."] LQ , }
};
}
