// Generated macro for class_bytes (function)
macro_rules! Depcrate_hirclass_bytes {
() => {
// Module: crate::hir
// Provides: {"class_bytes"}
// Dependencies: {}
# [doc = " Given a sequence of HIR values where each value corresponds to a byte class"] # [doc = " (or an all-ASCII Unicode class), return a single byte class corresponding"] # [doc = " to the union of the classes found."] fn class_bytes (hirs : & [Hir]) -> Option < Class > { let mut cls = ClassBytes :: new (vec ! []) ; for hir in hirs . iter () { match * hir . kind () { HirKind :: Class (Class :: Unicode (ref cls2)) => { cls . union (& cls2 . to_byte_class () ?) ; } HirKind :: Class (Class :: Bytes (ref cls2)) => { cls . union (cls2) ; } _ => return None , } ; } Some (Class :: Bytes (cls)) }
};
}
