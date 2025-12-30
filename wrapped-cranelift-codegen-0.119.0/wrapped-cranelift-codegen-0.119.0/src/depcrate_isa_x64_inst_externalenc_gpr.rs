// Generated macro for enc_gpr (function)
macro_rules! Depcrate_isa_x64_inst_externalenc_gpr {
() => {
// Module: crate::isa::x64::inst::external
// Provides: {"enc_gpr"}
// Dependencies: {}
# [doc = " A helper method for extracting the hardware encoding of a general purpose register."] # [inline] fn enc_gpr (gpr : & Gpr) -> u8 { if let Some (real) = gpr . to_reg () . to_real_reg () { real . hw_enc () } else { unreachable ! () } }
};
}
