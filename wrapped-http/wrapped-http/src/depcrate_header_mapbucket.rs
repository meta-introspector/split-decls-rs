// Generated macro for Bucket (struct)
macro_rules! Depcrate_header_mapBucket {
() => {
// Module: crate::header::map
// Provides: {"Bucket"}
// Dependencies: {}
# [doc = " Stores the data associated with a `HeaderMap` entry. Only the first value is"] # [doc = " included in this struct. If a header name has more than one associated"] # [doc = " value, all extra values are stored in the `extra_values` vector. A doubly"] # [doc = " linked list of entries is maintained. The doubly linked list is used so that"] # [doc = " removing a value is constant time. This also has the nice property of"] # [doc = " enabling double ended iteration."] # [derive (Debug , Clone)] struct Bucket < T > { hash : HashValue , key : HeaderName , value : T , links : Option < Links > , }
};
}
