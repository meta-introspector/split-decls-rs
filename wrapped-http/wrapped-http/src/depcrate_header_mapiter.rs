// Generated macro for Iter (struct)
macro_rules! Depcrate_header_mapIter {
() => {
// Module: crate::header::map
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " `HeaderMap` entry iterator."] # [doc = ""] # [doc = " Yields `(&HeaderName, &value)` tuples. The same header name may be yielded"] # [doc = " more than once if it has more than one associated value."] # [derive (Debug)] pub struct Iter < 'a , T > { map : & 'a HeaderMap < T > , entry : usize , cursor : Option < Cursor > , }
};
}
