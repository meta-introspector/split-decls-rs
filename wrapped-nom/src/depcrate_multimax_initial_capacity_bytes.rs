// Generated macro for MAX_INITIAL_CAPACITY_BYTES (const)
macro_rules! Depcrate_multiMAX_INITIAL_CAPACITY_BYTES {
() => {
// Module: crate::multi
// Provides: {"MAX_INITIAL_CAPACITY_BYTES"}
// Dependencies: {}
# [doc = " Don't pre-allocate more than 64KiB when calling `Vec::with_capacity`."] # [doc = ""] # [doc = " Pre-allocating memory is a nice optimization but count fields can't"] # [doc = " always be trusted. We should clamp initial capacities to some reasonable"] # [doc = " amount. This reduces the risk of a bogus count value triggering a panic"] # [doc = " due to an OOM error."] # [doc = ""] # [doc = " This does not affect correctness. Nom will always read the full number"] # [doc = " of elements regardless of the capacity cap."] # [cfg (feature = "alloc")] const MAX_INITIAL_CAPACITY_BYTES : usize = 65536 ;
};
}
