// Generated macro for IterMut (struct)
macro_rules! Depcrate_header_mapIterMut {
() => {
// Module: crate::header::map
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " `HeaderMap` mutable entry iterator"] # [doc = ""] # [doc = " Yields `(&HeaderName, &mut value)` tuples. The same header name may be"] # [doc = " yielded more than once if it has more than one associated value."] # [derive (Debug)] pub struct IterMut < 'a , T > { map : * mut HeaderMap < T > , entry : usize , cursor : Option < Cursor > , lt : PhantomData < & 'a mut HeaderMap < T > > , }
};
}
