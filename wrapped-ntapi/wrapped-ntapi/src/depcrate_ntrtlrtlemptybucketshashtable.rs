// Generated macro for RtlEmptyBucketsHashTable (function)
macro_rules! Depcrate_ntrtlRtlEmptyBucketsHashTable {
() => {
// Module: crate::ntrtl
// Provides: {"RtlEmptyBucketsHashTable"}
// Dependencies: {}
# [inline] pub const fn RtlEmptyBucketsHashTable (HashTable : & RTL_DYNAMIC_HASH_TABLE) -> ULONG { HashTable . TableSize - HashTable . NonEmptyBuckets }
};
}
