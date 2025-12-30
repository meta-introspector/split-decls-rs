// Generated macro for NANOSECONDS_PER_BASIC_BLOCK (const)
macro_rules! Depcrate_clockNANOSECONDS_PER_BASIC_BLOCK {
() => {
// Module: crate::clock
// Provides: {"NANOSECONDS_PER_BASIC_BLOCK"}
// Dependencies: {}
# [doc = " When using a virtual clock, this defines how many nanoseconds we pretend are passing for each"] # [doc = " basic block."] # [doc = " This number is pretty random, but it has been shown to approximately cause"] # [doc = " some sample programs to run within an order of magnitude of real time on desktop CPUs."] # [doc = " (See `tests/pass/shims/time-with-isolation*.rs`.)"] const NANOSECONDS_PER_BASIC_BLOCK : u128 = 5000 ;
};
}
