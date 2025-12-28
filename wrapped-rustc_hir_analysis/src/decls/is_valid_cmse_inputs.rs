macro_rules! is_valid_cmse_inputs {
    () => {
        # [doc = " Returns whether the inputs will fit into the available registers"] fn is_valid_cmse_inputs < 'tcx > (tcx : TyCtxt < 'tcx > , fn_sig : ty :: PolyFnSig < 'tcx > ,) -> Result < Result < () , usize > , & 'tcx LayoutError < 'tcx > > { let mut span = None ; let mut accum = 0u64 ; let fn_sig = tcx . instantiate_bound_regions_with_erased (fn_sig) ; for (index , ty) in fn_sig . inputs () . iter () . enumerate () { let layout = tcx . layout_of (ty :: TypingEnv :: fully_monomorphized () . as_query_input (* ty)) ? ; let align = layout . layout . align () . abi . bytes () ; let size = layout . layout . size () . bytes () ; accum += size ; accum = accum . next_multiple_of (Ord :: max (4 , align)) ; if accum > 16 { span = span . or (Some (index)) ; } } match span { None => Ok (Ok (())) , Some (span) => Ok (Err (span)) , } }
    };
}

is_valid_cmse_inputs!();