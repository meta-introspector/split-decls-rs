// Generated macro for ADDRS_PER_ANON_GLOBAL (const)
macro_rules! Depcrate_machineADDRS_PER_ANON_GLOBAL {
() => {
// Module: crate::machine
// Provides: {"ADDRS_PER_ANON_GLOBAL"}
// Dependencies: {}
# [doc = " Each anonymous global (constant, vtable, function pointer, ...) has multiple addresses, but only"] # [doc = " this many. Since const allocations are never deallocated, choosing a new [`AllocId`] and thus"] # [doc = " base address for each evaluation would produce unbounded memory usage."] const ADDRS_PER_ANON_GLOBAL : usize = 32 ;
};
}
