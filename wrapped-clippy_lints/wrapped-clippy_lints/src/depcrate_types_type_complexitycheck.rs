// Generated macro for check (function)
macro_rules! Depcrate_types_type_complexitycheck {
() => {
// Module: crate::types::type_complexity
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , ty : & hir :: Ty < '_ > , type_complexity_threshold : u64) -> bool { let score = { let mut visitor = TypeComplexityVisitor { score : 0 , nest : 1 } ; visitor . visit_ty_unambig (ty) ; visitor . score } ; if score > type_complexity_threshold { span_lint (cx , TYPE_COMPLEXITY , ty . span , "very complex type used. Consider factoring parts into `type` definitions" ,) ; true } else { false } }
};
}
