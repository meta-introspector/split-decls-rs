// Generated macro for IO_LOOP_BOUND (const)
macro_rules! DepcrateIO_LOOP_BOUND {
() => {
// Module: crate
// Provides: {"IO_LOOP_BOUND"}
// Dependencies: {}
# [doc = " Maximum number of datagrams processed in send/recv calls to make before moving on to other processing"] # [doc = ""] # [doc = " This helps ensure we don't starve anything when the CPU is slower than the link."] # [doc = " Value is selected by picking a low number which didn't degrade throughput in benchmarks."] const IO_LOOP_BOUND : usize = 160 ;
};
}
