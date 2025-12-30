// Generated macro for BenchRunResults (struct)
macro_rules! DepcrateBenchRunResults {
() => {
// Module: crate
// Provides: {"BenchRunResults"}
// Dependencies: {}
# [doc = " Result of a bench run."] struct BenchRunResults { allocation_attempts : usize , successful_allocations : usize , pre_fail_allocations : usize , deallocations : usize , # [doc = " Sorted vector of the amount of clock ticks per successful allocation."] all_alloc_measurements : Vec < u64 > , # [doc = " Sorted vector of the amount of clock ticks per successful allocation under heap pressure."] nofail_alloc_measurements : Vec < u64 > , # [doc = " Sorted vector of the amount of clock ticks per deallocation."] dealloc_measurements : Vec < u64 > , }
};
}
