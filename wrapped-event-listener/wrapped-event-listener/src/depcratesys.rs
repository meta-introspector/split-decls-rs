// Generated macro for sys (module)
macro_rules! Depcratesys {
() => {
// Module: crate
// Provides: {"sys"}
// Dependencies: {}
# [cfg_attr (any (feature = "std" , feature = "critical-section") , path = "intrusive.rs")] # [cfg_attr (not (any (feature = "std" , feature = "critical-section")) , path = "slab.rs")] mod sys ;
};
}
