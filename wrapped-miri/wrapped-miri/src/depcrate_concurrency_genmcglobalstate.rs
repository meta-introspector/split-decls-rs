// Generated macro for GlobalState (struct)
macro_rules! Depcrate_concurrency_genmcGlobalState {
() => {
// Module: crate::concurrency::genmc
// Provides: {"GlobalState"}
// Dependencies: {}
struct GlobalState { # [doc = " Keep track of global allocations, to ensure they keep the same address across different executions, even if the order of allocations changes."] # [doc = " The `AllocId` for globals is stable across executions, so we can use it as an identifier."] global_allocations : GlobalAllocationHandler , }
};
}
