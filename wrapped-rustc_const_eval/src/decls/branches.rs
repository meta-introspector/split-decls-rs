macro_rules! deps {
    () => {
        MPlaceTy!();
        CompileTimeInterpCx!();
    };
}

macro_rules! branches {
    () => {
        deps!();
        # [instrument (skip (ecx) , level = "debug")] fn branches < 'tcx > (ecx : & CompileTimeInterpCx < 'tcx > , place : & MPlaceTy < 'tcx > , field_count : usize , variant : Option < VariantIdx > , num_nodes : & mut usize ,) -> EvalToValTreeResult < 'tcx > { let place = match variant { Some (variant) => ecx . project_downcast (place , variant) . unwrap () , None => place . clone () , } ; debug ! (? place) ; let mut branches = Vec :: with_capacity (field_count + variant . is_some () as usize) ; if let Some (variant) = variant { branches . push (ty :: ValTree :: from_scalar_int (* ecx . tcx , variant . as_u32 () . into ())) ; } for i in 0 .. field_count { let field = ecx . project_field (& place , FieldIdx :: from_usize (i)) . unwrap () ; let valtree = const_to_valtree_inner (ecx , & field , num_nodes) ? ; branches . push (valtree) ; } if branches . len () == 0 { * num_nodes += 1 ; } Ok (ty :: ValTree :: from_branches (* ecx . tcx , branches)) }
    };
}

branches!()