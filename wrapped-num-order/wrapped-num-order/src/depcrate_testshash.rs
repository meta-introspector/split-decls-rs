// Generated macro for hash (function)
macro_rules! Depcrate_testshash {
() => {
// Module: crate::tests
// Provides: {"hash"}
// Dependencies: {}
fn hash (num : & N) -> u64 { let mut hasher = DefaultHasher :: new () ; num . hash (& mut hasher) ; hasher . finish () }
};
}
