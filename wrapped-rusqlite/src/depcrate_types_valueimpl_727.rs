// Generated macro for impl_727 (impl)
macro_rules! Depcrate_types_valueimpl_727 {
() => {
// Module: crate::types::value
// Provides: {"impl_727"}
// Dependencies: {}
# [cfg (feature = "uuid")] impl From < uuid :: Uuid > for Value { # [inline] fn from (id : uuid :: Uuid) -> Self { Self :: Blob (id . as_bytes () . to_vec ()) } }
};
}
