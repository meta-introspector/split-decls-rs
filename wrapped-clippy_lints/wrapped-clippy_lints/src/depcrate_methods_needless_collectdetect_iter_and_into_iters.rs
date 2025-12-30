// Generated macro for detect_iter_and_into_iters (function)
macro_rules! Depcrate_methods_needless_collectdetect_iter_and_into_iters {
() => {
// Module: crate::methods::needless_collect
// Provides: {"detect_iter_and_into_iters"}
// Dependencies: {}
# [doc = " Detect the occurrences of calls to `iter` or `into_iter` for the"] # [doc = " given identifier"] fn detect_iter_and_into_iters < 'tcx : 'a , 'a > (block : & 'tcx Block < 'tcx > , id : HirId , cx : & 'a LateContext < 'tcx > , captured_ids : HirIdSet ,) -> Option < Vec < IterFunction > > { let mut visitor = IterFunctionVisitor { illegal_mutable_capture_ids : captured_ids , current_mutably_captured_ids : HirIdSet :: default () , cx , uses : Vec :: new () , hir_id_uses_map : FxHashMap :: default () , current_statement_hir_id : None , seen_other : false , target : id , } ; visitor . visit_block (block) ; if visitor . seen_other { None } else { Some (visitor . uses . into_iter () . flatten () . collect ()) } }
};
}
