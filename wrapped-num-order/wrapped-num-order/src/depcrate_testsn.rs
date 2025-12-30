// Generated macro for N (enum)
macro_rules! Depcrate_testsN {
() => {
// Module: crate::tests
// Provides: {"N"}
// Dependencies: {}
# [derive (Clone , Debug)] # [allow (non_camel_case_types)] enum N { u8 (u8) , u16 (u16) , u32 (u32) , u64 (u64) , u128 (u128) , usize (usize) , i8 (i8) , i16 (i16) , i32 (i32) , i64 (i64) , i128 (i128) , isize (isize) , f32 (f32) , f64 (f64) , # [cfg (feature = "num-bigint")] ubig (BigUint) , # [cfg (feature = "num-bigint")] ibig (BigInt) , # [cfg (feature = "num-rational")] r8 (Ratio < i8 >) , # [cfg (feature = "num-rational")] r16 (Ratio < i16 >) , # [cfg (feature = "num-rational")] r32 (Ratio < i32 >) , # [cfg (feature = "num-rational")] r64 (Ratio < i64 >) , # [cfg (feature = "num-rational")] r128 (Ratio < i128 >) , # [cfg (feature = "num-rational")] rsize (Ratio < isize >) , # [cfg (all (feature = "num-bigint" , feature = "num-rational"))] rbig (Ratio < BigInt >) , # [cfg (feature = "num-complex")] c32 (Complex < f32 >) , # [cfg (feature = "num-complex")] c64 (Complex < f64 >) , }
};
}
