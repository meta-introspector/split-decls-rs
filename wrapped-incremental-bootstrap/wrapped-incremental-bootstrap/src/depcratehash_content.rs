// Generated macro for hash_content (function)
macro_rules! Depcratehash_content {
() => {
// Module: crate
// Provides: {"hash_content"}
// Dependencies: {}
fn hash_content (content : & str) -> u64 { let mut hasher = DefaultHasher :: new () ; content . hash (& mut hasher) ; hasher . finish () }
};
}
