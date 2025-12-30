// Generated macro for find_stmt_assigns_to (function)
macro_rules! Depcrate_redundant_clonefind_stmt_assigns_to {
() => {
// Module: crate::redundant_clone
// Provides: {"find_stmt_assigns_to"}
// Dependencies: {}
# [doc = " Finds the first `to = (&)from`, and returns"] # [doc = " ``Some((from, whether `from` cannot be moved out))``."] fn find_stmt_assigns_to < 'tcx > (cx : & LateContext < 'tcx > , mir : & mir :: Body < 'tcx > , to_local : mir :: Local , by_ref : bool , bb : mir :: BasicBlock ,) -> Option < (mir :: Local , CannotMoveOut) > { let rvalue = mir . basic_blocks [bb] . statements . iter () . rev () . find_map (| stmt | { if let mir :: StatementKind :: Assign (box (mir :: Place { local , .. } , v)) = & stmt . kind { return if * local == to_local { Some (v) } else { None } ; } None }) ? ; match (by_ref , rvalue) { (true , mir :: Rvalue :: Ref (_ , _ , place)) | (false , mir :: Rvalue :: Use (mir :: Operand :: Copy (place))) => { Some (base_local_and_movability (cx , mir , * place)) } , (false , mir :: Rvalue :: Ref (_ , _ , place)) => { if let [mir :: ProjectionElem :: Deref] = place . as_ref () . projection { Some (base_local_and_movability (cx , mir , * place)) } else { None } } , _ => None , } }
};
}
