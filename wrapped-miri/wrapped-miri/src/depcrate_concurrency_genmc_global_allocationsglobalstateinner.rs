// Generated macro for GlobalStateInner (struct)
macro_rules! Depcrate_concurrency_genmc_global_allocationsGlobalStateInner {
() => {
// Module: crate::concurrency::genmc::global_allocations
// Provides: {"GlobalStateInner"}
// Dependencies: {}
# [derive (Debug)] struct GlobalStateInner { # [doc = " The base address for each *global* allocation."] base_addr : FxHashMap < AllocId , u64 > , # [doc = " We use the same address generator that Miri uses in normal operation."] address_generator : AddressGenerator , # [doc = " The address generator needs an Rng to randomize the offsets between allocations."] # [doc = " We don't use the `MiriMachine` Rng since this is global, cross-machine state."] rng : StdRng , }
};
}
