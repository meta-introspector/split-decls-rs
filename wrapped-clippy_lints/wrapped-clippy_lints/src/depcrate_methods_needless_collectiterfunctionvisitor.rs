// Generated macro for IterFunctionVisitor (struct)
macro_rules! Depcrate_methods_needless_collectIterFunctionVisitor {
() => {
// Module: crate::methods::needless_collect
// Provides: {"IterFunctionVisitor"}
// Dependencies: {}
struct IterFunctionVisitor < 'a , 'tcx > { illegal_mutable_capture_ids : HirIdSet , current_mutably_captured_ids : HirIdSet , cx : & 'a LateContext < 'tcx > , uses : Vec < Option < IterFunction > > , hir_id_uses_map : FxHashMap < HirId , usize > , current_statement_hir_id : Option < HirId > , seen_other : bool , target : HirId , }
};
}
