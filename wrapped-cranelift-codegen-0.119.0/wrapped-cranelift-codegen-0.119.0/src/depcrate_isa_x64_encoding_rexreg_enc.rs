// Generated macro for reg_enc (function)
macro_rules! Depcrate_isa_x64_encoding_rexreg_enc {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"reg_enc"}
// Dependencies: {}
# [doc = " Get the encoding number of any register."] # [inline (always)] pub (crate) fn reg_enc (reg : impl Into < Reg >) -> u8 { let reg = reg . into () ; debug_assert ! (reg . is_real ()) ; reg . to_real_reg () . unwrap () . hw_enc () }
};
}
