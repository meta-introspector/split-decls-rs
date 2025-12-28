macro_rules! find_cold_blocks {
    () => {
        fn find_cold_blocks < 'tcx > (tcx : TyCtxt < 'tcx > , mir : & mir :: Body < 'tcx > ,) -> IndexVec < mir :: BasicBlock , bool > { let local_decls = & mir . local_decls ; let mut cold_blocks : IndexVec < mir :: BasicBlock , bool > = IndexVec :: from_elem (false , & mir . basic_blocks) ; for (bb , bb_data) in traversal :: postorder (mir) { let terminator = bb_data . terminator () ; match terminator . kind { mir :: TerminatorKind :: Call { ref func , .. } | mir :: TerminatorKind :: TailCall { ref func , .. } if let ty :: FnDef (def_id , ..) = * func . ty (local_decls , tcx) . kind () && let attrs = tcx . codegen_fn_attrs (def_id) && attrs . flags . contains (CodegenFnAttrFlags :: COLD) => { cold_blocks [bb] = true ; continue ; } mir :: TerminatorKind :: Unreachable => { cold_blocks [bb] = true ; continue ; } _ => { } } let mut succ = terminator . successors () ; if let Some (first) = succ . next () && cold_blocks [first] && succ . all (| s | cold_blocks [s]) { cold_blocks [bb] = true ; } } cold_blocks }
    };
}

find_cold_blocks!();