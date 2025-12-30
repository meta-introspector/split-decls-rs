// Generated macro for impl_399 (impl)
macro_rules! Depcrate_schema_metaimpl_399 {
() => {
// Module: crate::schema::meta
// Provides: {"impl_399"}
// Dependencies: {}
impl DeprecationStatus { # [doc = " If this deprecation status indicates the item is deprecated."] pub fn is_deprecated (& self) -> bool { match self { Self :: Current => false , Self :: Deprecated (_) => true , } } # [doc = " An optional reason for the deprecation, or none if `Current`."] pub fn reason (& self) -> Option < & ArcStr > { match self { Self :: Current => None , Self :: Deprecated (rsn) => rsn . as_ref () , } } }
};
}
