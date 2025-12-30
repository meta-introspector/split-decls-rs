// Generated macro for IsContiguous (trait)
macro_rules! Depcrate_base_storageIsContiguous {
() => {
// Module: crate::base::storage
// Provides: {"IsContiguous"}
// Dependencies: {}
# [doc = " Marker trait indicating that a storage is stored contiguously in memory."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The storage requirement means that for any value of `i` in `[0, nrows * ncols - 1]`, the value"] # [doc = " `.get_unchecked_linear` returns one of the matrix component. This trait is unsafe because"] # [doc = " failing to comply to this may cause Undefined Behaviors."] pub unsafe trait IsContiguous { }
};
}
