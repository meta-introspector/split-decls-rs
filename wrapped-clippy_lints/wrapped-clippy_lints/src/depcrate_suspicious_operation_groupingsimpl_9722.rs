// Generated macro for impl_9722 (impl)
macro_rules! Depcrate_suspicious_operation_groupingsimpl_9722 {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"impl_9722"}
// Dependencies: {}
impl EarlyLintPass for SuspiciousOperationGroupings { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { if expr . span . from_expansion () { return ; } if let Some (binops) = extract_related_binops (& expr . kind) { check_binops (cx , & binops . iter () . collect :: < Vec < _ > > ()) ; let mut op_types = Vec :: with_capacity (binops . len ()) ; binops . iter () . map (| b | b . op) . for_each (| op | { if ! op_types . contains (& op) { op_types . push (op) ; } }) ; for op_type in op_types { let ops : Vec < _ > = binops . iter () . filter (| b | b . op == op_type) . collect () ; check_binops (cx , & ops) ; } } } }
};
}
