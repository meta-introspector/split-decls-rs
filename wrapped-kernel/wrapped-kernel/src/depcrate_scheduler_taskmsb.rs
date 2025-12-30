// Generated macro for msb (function)
macro_rules! Depcrate_scheduler_taskmsb {
() => {
// Module: crate::scheduler::task
// Provides: {"msb"}
// Dependencies: {}
# [doc = " Returns the most significant bit."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " assert_eq!(msb(0), None);"] # [doc = " assert_eq!(msb(1), 0);"] # [doc = " assert_eq!(msb(u64::MAX), 63);"] # [doc = " ```"] # [inline] fn msb (n : u64) -> Option < u32 > { NonZeroU64 :: new (n) . map (| n | u64 :: BITS - 1 - n . leading_zeros ()) }
};
}
