// Generated macro for contains_unsafe_block (function)
macro_rules! Depcrate_visitorscontains_unsafe_block {
() => {
// Module: crate::visitors
// Provides: {"contains_unsafe_block"}
// Dependencies: {}
# [doc = " Checks if the given expression contains an unsafe block"] pub fn contains_unsafe_block < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < 'tcx >) -> bool { struct V < 'cx , 'tcx > { cx : & 'cx LateContext < 'tcx > , } impl < 'tcx > Visitor < 'tcx > for V < '_ , 'tcx > { type Result = ControlFlow < () > ; type NestedFilter = nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } fn visit_block (& mut self , b : & 'tcx Block < '_ >) -> Self :: Result { if b . rules == BlockCheckMode :: UnsafeBlock (UnsafeSource :: UserProvided) { ControlFlow :: Break (()) } else { walk_block (self , b) } } } let mut v = V { cx } ; v . visit_expr (e) . is_break () }
};
}
