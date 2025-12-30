// Generated macro for impl_40 (impl)
macro_rules! Depcrate_snapshot_utilimpl_40 {
() => {
// Module: crate::snapshot::util
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a > From < & 'a BStr > for EncodedString { fn from (v : & 'a BStr) -> Self { match v . to_str () { Ok (v) => EncodedString :: Utf8 (v . to_owned ()) , Err (_) => EncodedString :: Unknown (v . to_owned ()) , } } }
};
}
