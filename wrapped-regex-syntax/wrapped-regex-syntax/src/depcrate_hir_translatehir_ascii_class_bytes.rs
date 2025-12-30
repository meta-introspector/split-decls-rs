// Generated macro for hir_ascii_class_bytes (function)
macro_rules! Depcrate_hir_translatehir_ascii_class_bytes {
() => {
// Module: crate::hir::translate
// Provides: {"hir_ascii_class_bytes"}
// Dependencies: {}
fn hir_ascii_class_bytes (kind : & ast :: ClassAsciiKind) -> hir :: ClassBytes { let ranges : Vec < _ > = ascii_class (kind) . map (| (s , e) | hir :: ClassBytesRange :: new (s , e)) . collect () ; hir :: ClassBytes :: new (ranges) }
};
}
