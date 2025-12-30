// Generated macro for RawLinks (struct)
macro_rules! Depcrate_header_mapRawLinks {
() => {
// Module: crate::header::map
// Provides: {"RawLinks"}
// Dependencies: {}
# [doc = " Access to the `links` value in a slice of buckets."] # [doc = ""] # [doc = " It's important that no other field is accessed, since it may have been"] # [doc = " freed in a `Drain` iterator."] # [derive (Debug)] struct RawLinks < T > (* mut [Bucket < T >]) ;
};
}
