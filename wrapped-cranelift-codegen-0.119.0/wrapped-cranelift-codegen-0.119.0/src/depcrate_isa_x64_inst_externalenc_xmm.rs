// Generated macro for enc_xmm (function)
macro_rules! Depcrate_isa_x64_inst_externalenc_xmm {
() => {
// Module: crate::isa::x64::inst::external
// Provides: {"enc_xmm"}
// Dependencies: {}
# [doc = " A helper method for extracting the hardware encoding of an xmm register."] # [inline] fn enc_xmm (xmm : & Xmm) -> u8 { if let Some (real) = xmm . to_reg () . to_real_reg () { real . hw_enc () } else { unreachable ! () } }
};
}
