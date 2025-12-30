// Generated macro for AuxParams (struct)
macro_rules! Depcrate_significant_drop_tighteningAuxParams {
() => {
// Module: crate::significant_drop_tightening
// Provides: {"AuxParams"}
// Dependencies: {}
# [doc = " Auxiliary parameters used on each block check of an item"] struct AuxParams < 'others , 'stmt , 'tcx > { apas : & 'others mut FxIndexMap < HirId , AuxParamsAttr > , # [doc = " The current block identifier that is being visited."] curr_block_hir_id : HirId , # [doc = " The current block span that is being visited."] curr_block_span : Span , # [doc = " The current statement that is being visited."] curr_stmt : Cow < 'stmt , hir :: Stmt < 'tcx > > , }
};
}
