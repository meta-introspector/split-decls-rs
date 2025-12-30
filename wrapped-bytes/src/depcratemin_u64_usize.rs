// Generated macro for min_u64_usize (function)
macro_rules! Depcratemin_u64_usize {
() => {
// Module: crate
// Provides: {"min_u64_usize"}
// Dependencies: {}
# [inline (always)] # [cfg (feature = "std")] fn min_u64_usize (a : u64 , b : usize) -> usize { match usize :: try_from (a) { Ok (a) => usize :: min (a , b) , Err (_) => b , } }
};
}
