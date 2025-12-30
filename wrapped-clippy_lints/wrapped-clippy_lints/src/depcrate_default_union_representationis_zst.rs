// Generated macro for is_zst (function)
macro_rules! Depcrate_default_union_representationis_zst {
() => {
// Module: crate::default_union_representation
// Provides: {"is_zst"}
// Dependencies: {}
fn is_zst < 'tcx > (cx : & LateContext < 'tcx > , field : & FieldDef , args : ty :: GenericArgsRef < 'tcx >) -> bool { let ty = field . ty (cx . tcx , args) ; if let Ok (layout) = cx . layout_of (ty) { layout . is_zst () } else { false } }
};
}
