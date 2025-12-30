// Generated macro for impl_60 (impl)
macro_rules! Depcrate_util_flat_csvimpl_60 {
() => {
// Module: crate::util::flat_csv
// Provides: {"impl_60"}
// Dependencies: {}
impl < Sep : Separator > FromIterator < HeaderValue > for FlatCsv < Sep > { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = HeaderValue > , { let mut values = iter . into_iter () ; if let (1 , Some (1)) = values . size_hint () { return values . next () . expect ("size_hint claimed 1 item") . into () ; } let mut buf = values . next () . map (| val | BytesMut :: from (val . as_bytes ())) . unwrap_or_default () ; for val in values { buf . extend_from_slice (& [Sep :: BYTE , b' ']) ; buf . extend_from_slice (val . as_bytes ()) ; } let val = HeaderValue :: from_maybe_shared (buf . freeze ()) . expect ("comma separated HeaderValues are valid") ; val . into () } }
};
}
