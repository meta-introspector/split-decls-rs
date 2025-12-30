// Generated macro for extract_path (function)
macro_rules! Depcrate_use_trackingextract_path {
() => {
// Module: crate::use_tracking
// Provides: {"extract_path"}
// Dependencies: {}
fn extract_path (ty : & syn :: Type) -> Option < & syn :: TypePath > { if let syn :: Type :: Path (tpath) = ty { Some (tpath) } else { None } }
};
}
