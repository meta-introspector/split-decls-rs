// Generated macro for macro_13 (macro)
macro_rules! Depcratemacro_13 {
() => {
// Module: crate
// Provides: {"macro_13"}
// Dependencies: {}
float_reprs ! { Ieee16 (u16) { type RustcApFloat = rustc_apfloat :: ieee :: Half ; extern fn = cxx_apf_fuzz_eval_op_ieee16 ; } Ieee32 (u32) { type RustcApFloat = rustc_apfloat :: ieee :: Single ; extern fn = cxx_apf_fuzz_eval_op_ieee32 ; type HardFloat = f32 ; } Ieee64 (u64) { type RustcApFloat = rustc_apfloat :: ieee :: Double ; extern fn = cxx_apf_fuzz_eval_op_ieee64 ; type HardFloat = f64 ; } Ieee128 (u128) { type RustcApFloat = rustc_apfloat :: ieee :: Quad ; extern fn = cxx_apf_fuzz_eval_op_ieee128 ; } F8E5M2 (u8) { type RustcApFloat = rustc_apfloat :: ieee :: Float8E5M2 ; const REPR_TAG = 8 + 0 ; extern fn = cxx_apf_fuzz_eval_op_f8e5m2 ; } F8E4M3FN (u8) { type RustcApFloat = rustc_apfloat :: ieee :: Float8E4M3FN ; const REPR_TAG = 8 + 1 ; extern fn = cxx_apf_fuzz_eval_op_f8e4m3fn ; } BrainF16 (u16) { type RustcApFloat = rustc_apfloat :: ieee :: BFloat ; const REPR_TAG = 16 + 1 ; extern fn = cxx_apf_fuzz_eval_op_brainf16 ; } X87_F80 (u128) { type RustcApFloat = rustc_apfloat :: ieee :: X87DoubleExtended ; extern fn = cxx_apf_fuzz_eval_op_x87_f80 ; } }
};
}
