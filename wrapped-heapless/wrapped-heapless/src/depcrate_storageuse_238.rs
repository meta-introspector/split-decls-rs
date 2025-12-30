// Generated macro for use_238 (use)
macro_rules! Depcrate_storageuse_238 {
() => {
// Module: crate::storage
// Provides: {"use_238"}
// Dependencies: {}
# [cfg (any (feature = "portable-atomic" , all (feature = "mpmc_large" , target_has_atomic = "ptr") , all (not (feature = "mpmc_large") , target_has_atomic = "8")))] use crate :: mpmc ;
};
}
