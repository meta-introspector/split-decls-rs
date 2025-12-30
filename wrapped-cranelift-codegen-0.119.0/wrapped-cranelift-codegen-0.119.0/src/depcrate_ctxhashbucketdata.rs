// Generated macro for BucketData (struct)
macro_rules! Depcrate_ctxhashBucketData {
() => {
// Module: crate::ctxhash
// Provides: {"BucketData"}
// Dependencies: {}
# [doc = " A bucket in the hash table."] # [doc = ""] # [doc = " Some performance-related design notes: we cache the hashcode for"] # [doc = " speed, as this often buys a few percent speed in"] # [doc = " interning-table-heavy workloads. We only keep the low 32 bits of"] # [doc = " the hashcode, for memory efficiency: in common use, `K` and `V`"] # [doc = " are often 32 bits also, and a 12-byte bucket is measurably better"] # [doc = " than a 16-byte bucket."] struct BucketData < K , V > { hash : u32 , k : K , v : V , }
};
}
