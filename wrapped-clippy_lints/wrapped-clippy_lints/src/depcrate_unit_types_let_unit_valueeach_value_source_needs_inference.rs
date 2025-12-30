// Generated macro for each_value_source_needs_inference (function)
macro_rules! Depcrate_unit_types_let_unit_valueeach_value_source_needs_inference {
() => {
// Module: crate::unit_types::let_unit_value
// Provides: {"each_value_source_needs_inference"}
// Dependencies: {}
fn each_value_source_needs_inference (cx : & LateContext < '_ > , e : & Expr < '_ > , locals_to_check : & mut Vec < HirId > , seen_locals : & mut HirIdSet ,) -> bool { for_each_value_source (e , & mut | e | { if needs_inferred_result_ty (cx , e , locals_to_check , seen_locals) { ControlFlow :: Continue (()) } else { ControlFlow :: Break (()) } }) . is_continue () }
};
}
