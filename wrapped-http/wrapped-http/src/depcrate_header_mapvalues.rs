// Generated macro for Values (struct)
macro_rules! Depcrate_header_mapValues {
() => {
// Module: crate::header::map
// Provides: {"Values"}
// Dependencies: {}
# [doc = " `HeaderMap` value iterator."] # [doc = ""] # [doc = " Each value contained in the `HeaderMap` will be yielded."] # [derive (Debug)] pub struct Values < 'a , T > { inner : Iter < 'a , T > , }
};
}
