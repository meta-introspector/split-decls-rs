// Generated macro for timed_matmul (function)
macro_rules! Depcrate_matmultimed_matmul {
() => {
// Module: crate::matmul
// Provides: {"timed_matmul"}
// Dependencies: {}
fn timed_matmul < F : FnOnce (& [f32] , & [f32] , & mut [f32]) > (size : usize , f : F , name : & str) -> u64 { let size = size . next_power_of_two () ; let n = size * size ; let mut a = vec ! [0f32 ; n] ; let mut b = vec ! [0f32 ; n] ; for i in 0 .. n { a [i] = i as f32 ; b [i] = (i + 7) as f32 ; } let mut dest = vec ! [0f32 ; n] ; let start = Instant :: now () ; f (& a [..] , & b [..] , & mut dest [..]) ; let dur = Instant :: now () - start ; let nanos = u64 :: from (dur . subsec_nanos ()) + dur . as_secs () * 1_000_000_000u64 ; eprintln ! ("{}:\t{}x{} matrix: {} s" , name , size , size , nanos as f32 / 1e9f32) ; nanos }
};
}
