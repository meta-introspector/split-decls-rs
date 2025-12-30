// Generated macro for impl_4135 (impl)
macro_rules! Depcrate_manual_clampimpl_4135 {
() => {
// Module: crate::manual_clamp
// Provides: {"impl_4135"}
// Dependencies: {}
impl TypeClampability { fn is_clampable < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> Option < TypeClampability > { if ty . is_floating_point () { Some (TypeClampability :: Float) } else if cx . tcx . get_diagnostic_item (sym :: Ord) . is_some_and (| id | implements_trait (cx , ty , id , & [])) { Some (TypeClampability :: Ord) } else { None } } fn is_float (self) -> bool { matches ! (self , TypeClampability :: Float) } }
};
}
