// Generated macro for impl_38 (impl)
macro_rules! Depcrate_snapshot_utilimpl_38 {
() => {
// Module: crate::snapshot::util
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a > From < & 'a BStr > for EncodedStringRef < 'a > { fn from (v : & 'a BStr) -> Self { match v . to_str () { Ok (v) => EncodedStringRef :: Utf8 (v) , Err (_) => EncodedStringRef :: Unknown (v) , } } }
};
}
