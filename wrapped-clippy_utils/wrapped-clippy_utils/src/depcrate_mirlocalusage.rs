// Generated macro for LocalUsage (struct)
macro_rules! Depcrate_mirLocalUsage {
() => {
// Module: crate::mir
// Provides: {"LocalUsage"}
// Dependencies: {}
# [derive (Clone , Debug , Default)] pub struct LocalUsage { # [doc = " The locations where the local is used, if any."] pub local_use_locs : Vec < Location > , # [doc = " The locations where the local is consumed or mutated, if any."] pub local_consume_or_mutate_locs : Vec < Location > , }
};
}
