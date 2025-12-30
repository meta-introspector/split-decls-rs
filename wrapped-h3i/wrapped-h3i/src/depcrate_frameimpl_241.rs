// Generated macro for impl_241 (impl)
macro_rules! Depcrate_frameimpl_241 {
() => {
// Module: crate::frame
// Provides: {"impl_241"}
// Dependencies: {}
impl From < Vec < Header > > for EnrichedHeaders { fn from (headers : Vec < Header >) -> Self { let header_block = encode_header_block (& headers) . unwrap () ; let mut header_map : HeaderMap = MultiMap :: with_capacity (headers . len ()) ; for header in headers . iter () { header_map . insert (header . name () . to_vec () , header . value () . to_vec ()) ; } Self { header_block , headers , header_map , } } }
};
}
