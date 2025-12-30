// Generated macro for GlobalAllocationHandler (struct)
macro_rules! Depcrate_concurrency_genmc_global_allocationsGlobalAllocationHandler {
() => {
// Module: crate::concurrency::genmc::global_allocations
// Provides: {"GlobalAllocationHandler"}
// Dependencies: {}
# [doc = " Allocator for global memory in GenMC mode."] # [doc = " Miri doesn't discover all global allocations statically like LLI does for GenMC."] # [doc = " The existing global memory allocator in GenMC doesn't support this, so we take over these allocations."] # [doc = " Global allocations need to be in a specific address range, with the lower limit given by the `GENMC_GLOBAL_ADDRESSES_MASK` constant."] # [doc = ""] # [doc = " Every global allocation must have the same addresses across all executions of a single program."] # [doc = " Therefore there is only 1 global allocator, and it syncs new globals across executions, even if they are explored in parallel."] # [derive (Debug)] pub struct GlobalAllocationHandler (RwLock < GlobalStateInner >) ;
};
}
