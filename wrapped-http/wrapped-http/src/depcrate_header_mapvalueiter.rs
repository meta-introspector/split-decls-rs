// Generated macro for ValueIter (struct)
macro_rules! Depcrate_header_mapValueIter {
() => {
// Module: crate::header::map
// Provides: {"ValueIter"}
// Dependencies: {}
# [doc = " An iterator of all values associated with a single header name."] # [derive (Debug)] pub struct ValueIter < 'a , T > { map : & 'a HeaderMap < T > , index : usize , front : Option < Cursor > , back : Option < Cursor > , }
};
}
