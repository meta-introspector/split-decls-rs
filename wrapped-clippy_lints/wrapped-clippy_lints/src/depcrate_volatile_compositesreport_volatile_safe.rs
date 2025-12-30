// Generated macro for report_volatile_safe (function)
macro_rules! Depcrate_volatile_compositesreport_volatile_safe {
() => {
// Module: crate::volatile_composites
// Provides: {"report_volatile_safe"}
// Dependencies: {}
# [doc = " Print diagnostic for volatile read/write on non-volatile-safe types."] fn report_volatile_safe < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx > , ty : Ty < 'tcx >) { if ! is_volatile_safe_ty (cx , ty) { span_lint (cx , VOLATILE_COMPOSITES , expr . span , format ! ("type `{ty}` is not volatile-compatible") ,) ; } }
};
}
