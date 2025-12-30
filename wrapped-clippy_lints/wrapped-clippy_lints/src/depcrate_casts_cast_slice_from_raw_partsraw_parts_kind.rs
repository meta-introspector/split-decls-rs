// Generated macro for raw_parts_kind (function)
macro_rules! Depcrate_casts_cast_slice_from_raw_partsraw_parts_kind {
() => {
// Module: crate::casts::cast_slice_from_raw_parts
// Provides: {"raw_parts_kind"}
// Dependencies: {}
fn raw_parts_kind (cx : & LateContext < '_ > , did : DefId) -> Option < RawPartsKind > { match cx . tcx . get_diagnostic_name (did) ? { sym :: slice_from_raw_parts => Some (RawPartsKind :: Immutable) , sym :: slice_from_raw_parts_mut => Some (RawPartsKind :: Mutable) , _ => None , } }
};
}
