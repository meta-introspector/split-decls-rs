// Generated macro for is_potentially_mutated (function)
macro_rules! Depcrate_usageis_potentially_mutated {
() => {
// Module: crate::usage
// Provides: {"is_potentially_mutated"}
// Dependencies: {}
pub fn is_potentially_mutated < 'tcx > (variable : HirId , expr : & 'tcx Expr < '_ > , cx : & LateContext < 'tcx >) -> bool { mutated_variables (expr , cx) . is_none_or (| mutated | mutated . contains (& variable)) }
};
}
