// Generated macro for std_vec (module)
macro_rules! Depcrate_ser_flavorsstd_vec {
() => {
// Module: crate::ser::flavors
// Provides: {"std_vec"}
// Dependencies: {}
# [cfg (feature = "use-std")] mod std_vec { # [doc = " The `StdVec` flavor is a wrapper type around a `std::vec::Vec`."] # [doc = ""] # [doc = " This type is only available when the (non-default) `use-std` feature is active"] pub type StdVec = super :: alloc_vec :: AllocVec ; }
};
}
