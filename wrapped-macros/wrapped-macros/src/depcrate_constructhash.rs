// Generated macro for hash (function)
macro_rules! Depcrate_constructhash {
() => {
// Module: crate::construct
// Provides: {"hash"}
// Dependencies: {}
fn hash (string : & str) -> u64 { let mut hasher = DefaultHasher :: new () ; string . hash (& mut hasher) ; hasher . finish () }
};
}
