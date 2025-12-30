// Generated macro for class_chars (function)
macro_rules! Depcrate_hirclass_chars {
() => {
// Module: crate::hir
// Provides: {"class_chars"}
// Dependencies: {}
# [doc = " Given a sequence of HIR values where each value corresponds to a Unicode"] # [doc = " class (or an all-ASCII byte class), return a single Unicode class"] # [doc = " corresponding to the union of the classes found."] fn class_chars (hirs : & [Hir]) -> Option < Class > { let mut cls = ClassUnicode :: new (vec ! []) ; for hir in hirs . iter () { match * hir . kind () { HirKind :: Class (Class :: Unicode (ref cls2)) => { cls . union (cls2) ; } HirKind :: Class (Class :: Bytes (ref cls2)) => { cls . union (& cls2 . to_unicode_class () ?) ; } _ => return None , } ; } Some (Class :: Unicode (cls)) }
};
}
