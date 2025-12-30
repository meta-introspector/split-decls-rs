// Generated macro for Keys (struct)
macro_rules! Depcrate_header_mapKeys {
() => {
// Module: crate::header::map
// Provides: {"Keys"}
// Dependencies: {}
# [doc = " An iterator over `HeaderMap` keys."] # [doc = ""] # [doc = " Each header name is yielded only once, even if it has more than one"] # [doc = " associated value."] # [derive (Debug)] pub struct Keys < 'a , T > { inner : :: std :: slice :: Iter < 'a , Bucket < T > > , }
};
}
