// Generated macro for impl_238 (impl)
macro_rules! Depcrate_responseimpl_238 {
() => {
// Module: crate::response
// Provides: {"impl_238"}
// Dependencies: {}
impl < M > DataPayloadOr < M , () > where M : DynamicDataMarker , { # [doc = " Convenience function to return the other type with value `()`"] # [inline] pub fn none () -> Self { Self :: from_other (()) } # [doc = " Convenience function to return `Some` or `None` for other type `()`"] # [inline] pub fn get_option < 'a > (& 'a self) -> Option < & 'a < M :: DataStruct as Yokeable < 'a > > :: Output > { self . get () . ok () } }
};
}
