// Generated macro for create_valtree_place (function)
macro_rules! Depcrate_const_eval_valtreescreate_valtree_place {
() => {
// Module: crate::const_eval::valtrees
// Provides: {"create_valtree_place"}
// Dependencies: {}
# [instrument (skip (ecx) , level = "debug" , ret)] fn create_valtree_place < 'tcx > (ecx : & mut CompileTimeInterpCx < 'tcx > , layout : TyAndLayout < 'tcx > , valtree : ty :: ValTree < 'tcx > ,) -> MPlaceTy < 'tcx > { let meta = reconstruct_place_meta (layout , valtree , ecx . tcx . tcx) ; ecx . allocate_dyn (layout , MemoryKind :: Stack , meta) . unwrap () }
};
}
