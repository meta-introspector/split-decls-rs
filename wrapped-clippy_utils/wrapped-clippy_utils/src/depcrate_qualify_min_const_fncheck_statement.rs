// Generated macro for check_statement (function)
macro_rules! Depcrate_qualify_min_const_fncheck_statement {
() => {
// Module: crate::qualify_min_const_fn
// Provides: {"check_statement"}
// Dependencies: {}
fn check_statement < 'tcx > (cx : & LateContext < 'tcx > , body : & Body < 'tcx > , def_id : DefId , statement : & Statement < 'tcx > , msrv : Msrv ,) -> McfResult { let span = statement . source_info . span ; match & statement . kind { StatementKind :: Assign (box (place , rval)) => { check_place (cx , * place , span , body , msrv) ? ; check_rvalue (cx , body , def_id , rval , span , msrv) } , StatementKind :: FakeRead (box (_ , place)) => check_place (cx , * place , span , body , msrv) , StatementKind :: SetDiscriminant { place , .. } => check_place (cx , * * place , span , body , msrv) , StatementKind :: Intrinsic (box NonDivergingIntrinsic :: Assume (op)) => check_operand (cx , op , span , body , msrv) , StatementKind :: Intrinsic (box NonDivergingIntrinsic :: CopyNonOverlapping (rustc_middle :: mir :: CopyNonOverlapping { dst , src , count } ,)) => { check_operand (cx , dst , span , body , msrv) ? ; check_operand (cx , src , span , body , msrv) ? ; check_operand (cx , count , span , body , msrv) } , StatementKind :: StorageLive (_) | StatementKind :: StorageDead (_) | StatementKind :: Retag { .. } | StatementKind :: AscribeUserType (..) | StatementKind :: PlaceMention (..) | StatementKind :: Coverage (..) | StatementKind :: ConstEvalCounter | StatementKind :: BackwardIncompatibleDropHint { .. } | StatementKind :: Nop => Ok (()) , } }
};
}
