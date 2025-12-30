// Generated macro for hash_span (function)
macro_rules! Depcratehash_span {
() => {
// Module: crate
// Provides: {"hash_span"}
// Dependencies: {}
# [doc = " A helper to hash the item's span for a somewhat unique identifier."] fn hash_span (span : proc_macro2 :: Span) -> u64 { let mut hasher = DefaultHasher :: new () ; format ! ("{:?}" , span) . hash (& mut hasher) ; hasher . finish () }
};
}
