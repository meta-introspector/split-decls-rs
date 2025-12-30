// Generated macro for record_builder (function)
macro_rules! Depcrate_gv_recordrecord_builder {
() => {
// Module: crate::gv::record
// Provides: {"record_builder"}
// Dependencies: {}
pub fn record_builder (label : & str) -> ShapeKind { let res = parse_record_string (label) ; ShapeKind :: Record (res) }
};
}
