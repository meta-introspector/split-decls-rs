// Generated macro for HashMapExt (trait)
macro_rules! DepcrateHashMapExt {
() => {
// Module: crate
// Provides: {"HashMapExt"}
// Dependencies: {}
# [cfg (feature = "std")] # [doc = " A convenience trait that can be used together with the type aliases defined to"] # [doc = " get access to the `new()` and `with_capacity()` methods for the HashMap type alias."] pub trait HashMapExt { # [doc = " Constructs a new HashMap"] fn new () -> Self ; # [doc = " Constructs a new HashMap with a given initial capacity"] fn with_capacity (capacity : usize) -> Self ; }
};
}
