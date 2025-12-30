// Generated macro for impl_109 (impl)
macro_rules! Depcrate_util_value_stringimpl_109 {
() => {
// Module: crate::util::value_string
// Provides: {"impl_109"}
// Dependencies: {}
impl HeaderValueString { pub (crate) fn from_val (val : & HeaderValue) -> Result < Self , Error > { if val . to_str () . is_ok () { Ok (HeaderValueString { value : val . clone () }) } else { Err (Error :: invalid ()) } } pub (crate) fn from_string (src : String) -> Option < Self > { let bytes = Bytes :: from (src) ; HeaderValue :: from_maybe_shared (bytes) . ok () . map (| value | HeaderValueString { value }) } pub (crate) const fn from_static (src : & 'static str) -> HeaderValueString { HeaderValueString { value : HeaderValue :: from_static (src) , } } pub (crate) fn as_str (& self) -> & str { unsafe { str :: from_utf8_unchecked (self . value . as_bytes ()) } } }
};
}
