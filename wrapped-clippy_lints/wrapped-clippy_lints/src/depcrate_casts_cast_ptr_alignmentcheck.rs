// Generated macro for check (function)
macro_rules! Depcrate_casts_cast_ptr_alignmentcheck {
() => {
// Module: crate::casts::cast_ptr_alignment
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < '_ > , cast_from : Ty < 'tcx > , cast_to : Ty < 'tcx >) { lint_cast_ptr_alignment (cx , expr , cast_from , cast_to) ; }
};
}
