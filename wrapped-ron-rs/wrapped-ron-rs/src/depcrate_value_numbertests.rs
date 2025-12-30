// Generated macro for tests (module)
macro_rules! Depcrate_value_numbertests {
() => {
// Module: crate::value::number
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_nan () { assert_eq ! (F32 (f32 :: NAN) , F32 (f32 :: NAN)) ; assert_eq ! (F32 (- f32 :: NAN) , F32 (- f32 :: NAN)) ; assert_ne ! (F32 (f32 :: NAN) , F32 (- f32 :: NAN)) ; } # [cfg (feature = "std")] # [test] fn test_nan_hash () { use std :: collections :: hash_map :: DefaultHasher ; use std :: hash :: { Hash , Hasher } ; fn hash < T : Hash > (v : & T) -> u64 { let mut state = DefaultHasher :: new () ; v . hash (& mut state) ; state . finish () } assert_eq ! (hash (& F32 (f32 :: NAN)) , hash (& F32 (f32 :: NAN))) ; assert_eq ! (hash (& F32 (- f32 :: NAN)) , hash (& F32 (- f32 :: NAN))) ; assert_ne ! (hash (& F32 (f32 :: NAN)) , hash (& F32 (- f32 :: NAN))) ; } # [test] fn test_partial_ord () { assert ! (F32 (f32 :: NAN) > F32 (f32 :: INFINITY)) ; assert ! (F32 (- f32 :: NAN) < F32 (f32 :: NEG_INFINITY)) ; assert ! (F32 (f32 :: NAN) == F32 (f32 :: NAN)) ; } }
};
}
