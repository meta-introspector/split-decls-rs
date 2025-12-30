// Generated macro for impl_186 (impl)
macro_rules! Depcrate_header_nameimpl_186 {
() => {
// Module: crate::header::name
// Provides: {"impl_186"}
// Dependencies: {}
# [doc (hidden)] impl < 'a > From < HdrName < 'a > > for HeaderName { fn from (src : HdrName < 'a >) -> HeaderName { match src . inner { Repr :: Standard (s) => HeaderName { inner : Repr :: Standard (s) , } , Repr :: Custom (maybe_lower) => { if maybe_lower . lower { let buf = Bytes :: copy_from_slice (maybe_lower . buf) ; let byte_str = unsafe { ByteStr :: from_utf8_unchecked (buf) } ; HeaderName { inner : Repr :: Custom (Custom (byte_str)) , } } else { use bytes :: BufMut ; let mut dst = BytesMut :: with_capacity (maybe_lower . buf . len ()) ; for b in maybe_lower . buf . iter () { dst . put_u8 (HEADER_CHARS [* b as usize]) ; } let buf = unsafe { ByteStr :: from_utf8_unchecked (dst . freeze ()) } ; HeaderName { inner : Repr :: Custom (Custom (buf)) , } } } } } }
};
}
