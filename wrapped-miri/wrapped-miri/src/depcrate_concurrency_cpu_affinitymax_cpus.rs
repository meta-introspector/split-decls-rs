// Generated macro for MAX_CPUS (const)
macro_rules! Depcrate_concurrency_cpu_affinityMAX_CPUS {
() => {
// Module: crate::concurrency::cpu_affinity
// Provides: {"MAX_CPUS"}
// Dependencies: {}
# [doc = " The maximum number of CPUs supported by miri."] # [doc = ""] # [doc = " This value is compatible with the libc `CPU_SETSIZE` constant and corresponds to the number"] # [doc = " of CPUs that a `cpu_set_t` can contain."] # [doc = ""] # [doc = " Real machines can have more CPUs than this number, and there exist APIs to set their affinity,"] # [doc = " but this is not currently supported by miri."] pub const MAX_CPUS : usize = 1024 ;
};
}
