// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__alloc_hashtest {
() => {
// Module: crate::arbitrary::_alloc::hash
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { # [cfg (feature = "std")] no_panic_test ! (default_hasher => DefaultHasher , random_state => RandomState , build_hasher_default => BuildHasherDefault < DefaultHasher >) ; }
};
}
