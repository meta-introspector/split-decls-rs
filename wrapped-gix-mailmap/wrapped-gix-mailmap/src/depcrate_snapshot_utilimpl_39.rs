// Generated macro for impl_39 (impl)
macro_rules! Depcrate_snapshot_utilimpl_39 {
() => {
// Module: crate::snapshot::util
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'a > From < EncodedStringRef < 'a > > for EncodedString { fn from (v : EncodedStringRef < 'a >) -> Self { match v { EncodedStringRef :: Utf8 (v) => EncodedString :: Utf8 (v . to_owned ()) , EncodedStringRef :: Unknown (v) => EncodedString :: Unknown (v . to_owned ()) , } } }
};
}
