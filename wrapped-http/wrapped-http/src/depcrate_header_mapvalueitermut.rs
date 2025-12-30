// Generated macro for ValueIterMut (struct)
macro_rules! Depcrate_header_mapValueIterMut {
() => {
// Module: crate::header::map
// Provides: {"ValueIterMut"}
// Dependencies: {}
# [doc = " A mutable iterator of all values associated with a single header name."] # [derive (Debug)] pub struct ValueIterMut < 'a , T > { map : * mut HeaderMap < T > , index : usize , front : Option < Cursor > , back : Option < Cursor > , lt : PhantomData < & 'a mut HeaderMap < T > > , }
};
}
