// Generated macro for impl_752 (impl)
macro_rules! Depcrate_types_value_refimpl_752 {
() => {
// Module: crate::types::value_ref
// Provides: {"impl_752"}
// Dependencies: {}
impl < 'a > From < & 'a Value > for ValueRef < 'a > { # [inline] fn from (value : & 'a Value) -> Self { match * value { Value :: Null => ValueRef :: Null , Value :: Integer (i) => ValueRef :: Integer (i) , Value :: Real (r) => ValueRef :: Real (r) , Value :: Text (ref s) => ValueRef :: Text (s . as_bytes ()) , Value :: Blob (ref b) => ValueRef :: Blob (b) , } } }
};
}
