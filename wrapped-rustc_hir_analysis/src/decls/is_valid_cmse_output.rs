macro_rules! is_valid_cmse_output {
    () => {
        # [doc = " Returns whether the output will fit into the available registers"] fn is_valid_cmse_output < 'tcx > (tcx : TyCtxt < 'tcx > , fn_sig : ty :: PolyFnSig < 'tcx > ,) -> Result < bool , & 'tcx LayoutError < 'tcx > > { let fn_sig = tcx . instantiate_bound_regions_with_erased (fn_sig) ; let typing_env = ty :: TypingEnv :: fully_monomorphized () ; let mut ret_ty = fn_sig . output () ; let layout = tcx . layout_of (typing_env . as_query_input (ret_ty)) ? ; let size = layout . layout . size () . bytes () ; if size <= 4 { return Ok (true) ; } else if size > 8 { return Ok (false) ; } 'outer : loop { let ty :: Adt (adt_def , args) = ret_ty . kind () else { break ; } ; if ! adt_def . repr () . transparent () { break ; } for variant_def in adt_def . variants () { for field_def in variant_def . fields . iter () { let ty = field_def . ty (tcx , args) ; let layout = tcx . layout_of (typing_env . as_query_input (ty)) ? ; if ! layout . layout . is_1zst () { ret_ty = ty ; continue 'outer ; } } } } Ok (ret_ty == tcx . types . i64 || ret_ty == tcx . types . u64 || ret_ty == tcx . types . f64) }
    };
}

is_valid_cmse_output!();