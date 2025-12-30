// Generated macro for int_reg_enc (function)
macro_rules! Depcrate_isa_x64_encoding_rexint_reg_enc {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"int_reg_enc"}
// Dependencies: {}
# [doc = " Get the encoding number of a GPR."] # [inline (always)] pub (crate) fn int_reg_enc (reg : impl Into < Reg >) -> u8 { let reg = reg . into () ; debug_assert ! (reg . is_real () , "reg = {reg:?}") ; debug_assert_eq ! (reg . class () , RegClass :: Int) ; reg . to_real_reg () . unwrap () . hw_enc () }
};
}
