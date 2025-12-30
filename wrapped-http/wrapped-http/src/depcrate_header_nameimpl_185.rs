// Generated macro for impl_185 (impl)
macro_rules! Depcrate_header_nameimpl_185 {
() => {
// Module: crate::header::name
// Provides: {"impl_185"}
// Dependencies: {}
impl < 'a > HdrName < 'a > { fn custom (buf : & 'a [u8] , lower : bool) -> HdrName < 'a > { HdrName { inner : Repr :: Custom (MaybeLower { buf , lower }) , } } pub fn from_bytes < F , U > (hdr : & [u8] , f : F) -> Result < U , InvalidHeaderName > where F : FnOnce (HdrName < '_ >) -> U , { let mut buf = uninit_u8_array () ; let hdr = parse_hdr (hdr , & mut buf , & HEADER_CHARS) ? ; Ok (f (hdr)) } pub fn from_static < F , U > (hdr : & 'static str , f : F) -> U where F : FnOnce (HdrName < '_ >) -> U , { let mut buf = uninit_u8_array () ; let hdr = parse_hdr (hdr . as_bytes () , & mut buf , & HEADER_CHARS) . expect ("static str is invalid name") ; f (hdr) } }
};
}
