// Generated macro for hir_class (function)
macro_rules! Depcrate_unicodehir_class {
() => {
// Module: crate::unicode
// Provides: {"hir_class"}
// Dependencies: {}
# [doc = " Build a Unicode HIR class from a sequence of Unicode scalar value ranges."] pub fn hir_class (ranges : & [(char , char)]) -> hir :: ClassUnicode { let hir_ranges : Vec < hir :: ClassUnicodeRange > = ranges . iter () . map (| & (s , e) | hir :: ClassUnicodeRange :: new (s , e)) . collect () ; hir :: ClassUnicode :: new (hir_ranges) }
};
}
