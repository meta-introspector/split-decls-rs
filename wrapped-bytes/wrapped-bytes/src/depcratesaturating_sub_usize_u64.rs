// Generated macro for saturating_sub_usize_u64 (function)
macro_rules! Depcratesaturating_sub_usize_u64 {
() => {
// Module: crate
// Provides: {"saturating_sub_usize_u64"}
// Dependencies: {}
# [inline (always)] # [cfg (feature = "std")] fn saturating_sub_usize_u64 (a : usize , b : u64) -> usize { match usize :: try_from (b) { Ok (b) => a . saturating_sub (b) , Err (_) => 0 , } }
};
}
