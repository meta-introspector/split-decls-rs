// Generated macro for Constant (enum)
macro_rules! Depcrate_constsConstant {
() => {
// Module: crate::consts
// Provides: {"Constant"}
// Dependencies: {}
# [doc = " A `LitKind`-like enum to fold constant `Expr`s into."] # [derive (Debug , Clone)] pub enum Constant { Adt (ConstValue) , # [doc = " A `String` (e.g., \"abc\")."] Str (String) , # [doc = " A binary string (e.g., `b\"abc\"`)."] Binary (Vec < u8 >) , # [doc = " A single `char` (e.g., `'a'`)."] Char (char) , # [doc = " An integer's bit representation."] Int (u128) , # [doc = " An `f16` bitcast to a `u16`."] F16 (u16) , # [doc = " An `f32`."] F32 (f32) , # [doc = " An `f64`."] F64 (f64) , # [doc = " An `f128` bitcast to a `u128`."] F128 (u128) , # [doc = " `true` or `false`."] Bool (bool) , # [doc = " An array of constants."] Vec (Vec < Self >) , # [doc = " Also an array, but with only one constant, repeated N times."] Repeat (Box < Self > , u64) , # [doc = " A tuple of constants."] Tuple (Vec < Self >) , # [doc = " A raw pointer."] RawPtr (u128) , # [doc = " A reference"] Ref (Box < Self >) , # [doc = " A literal with syntax error."] Err , }
};
}
