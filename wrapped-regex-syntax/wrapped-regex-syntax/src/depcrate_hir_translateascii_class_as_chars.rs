// Generated macro for ascii_class_as_chars (function)
macro_rules! Depcrate_hir_translateascii_class_as_chars {
() => {
// Module: crate::hir::translate
// Provides: {"ascii_class_as_chars"}
// Dependencies: {}
fn ascii_class_as_chars (kind : & ast :: ClassAsciiKind ,) -> impl Iterator < Item = (char , char) > { ascii_class (kind) . map (| (s , e) | (char :: from (s) , char :: from (e))) }
};
}
