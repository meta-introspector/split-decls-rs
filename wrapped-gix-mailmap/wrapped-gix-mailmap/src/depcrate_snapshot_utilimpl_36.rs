// Generated macro for impl_36 (impl)
macro_rules! Depcrate_snapshot_utilimpl_36 {
() => {
// Module: crate::snapshot::util
// Provides: {"impl_36"}
// Dependencies: {}
impl EncodedString { pub fn as_bstr (& self) -> & BStr { match self { EncodedString :: Utf8 (v) => v . as_str () . into () , EncodedString :: Unknown (v) => v . as_bstr () , } } pub fn cmp_ref (& self , other : EncodedStringRef < '_ >) -> Ordering { match (self , other) { (EncodedString :: Utf8 (a) , EncodedStringRef :: Utf8 (b)) => { let a = a . chars () . map (| c | c . to_ascii_lowercase ()) ; let b = b . chars () . map (| c | c . to_ascii_lowercase ()) ; a . cmp (b) } (EncodedString :: Unknown (a) , EncodedStringRef :: Unknown (b)) => a . deref () . as_bstr () . cmp (b) , (EncodedString :: Utf8 (a) , EncodedStringRef :: Unknown (b)) => a . as_bytes () . cmp (b . as_ref ()) , (EncodedString :: Unknown (a) , EncodedStringRef :: Utf8 (b)) => a . deref () . as_bytes () . cmp (b . as_bytes ()) , } } }
};
}
