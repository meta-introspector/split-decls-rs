macro_rules! deps {
    () => {
        MPlaceTy!();
        CompileTimeInterpCx!();
    };
}

macro_rules! slice_branches {
    () => {
        deps!();
        # [instrument (skip (ecx) , level = "debug")] fn slice_branches < 'tcx > (ecx : & CompileTimeInterpCx < 'tcx > , place : & MPlaceTy < 'tcx > , num_nodes : & mut usize ,) -> EvalToValTreeResult < 'tcx > { let n = place . len (ecx) . unwrap_or_else (| _ | panic ! ("expected to use len of place {place:?}")) ; let mut elems = Vec :: with_capacity (n as usize) ; for i in 0 .. n { let place_elem = ecx . project_index (place , i) . unwrap () ; let valtree = const_to_valtree_inner (ecx , & place_elem , num_nodes) ? ; elems . push (valtree) ; } Ok (ty :: ValTree :: from_branches (* ecx . tcx , elems)) }
    };
}

slice_branches!();