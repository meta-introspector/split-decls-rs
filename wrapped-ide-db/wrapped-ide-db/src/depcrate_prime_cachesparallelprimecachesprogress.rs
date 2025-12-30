// Generated macro for ParallelPrimeCachesProgress (struct)
macro_rules! Depcrate_prime_cachesParallelPrimeCachesProgress {
() => {
// Module: crate::prime_caches
// Provides: {"ParallelPrimeCachesProgress"}
// Dependencies: {}
# [doc = " We're indexing many crates."] # [derive (Debug)] pub struct ParallelPrimeCachesProgress { # [doc = " the crates that we are currently priming."] pub crates_currently_indexing : Vec < Symbol > , # [doc = " the total number of crates we want to prime."] pub crates_total : usize , # [doc = " the total number of crates that have finished priming"] pub crates_done : usize , pub work_type : & 'static str , }
};
}
