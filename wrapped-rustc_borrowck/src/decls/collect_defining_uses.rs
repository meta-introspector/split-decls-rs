macro_rules! deps {
    () => {
        DeferredOpaqueTypeError!();
        RegionCtxt!();
        DefiningUse!();
    };
}

macro_rules! collect_defining_uses {
    () => {
        deps!();
        # [instrument (level = "debug" , skip_all , ret)] fn collect_defining_uses < 'tcx > (rcx : & mut RegionCtxt < '_ , 'tcx > , concrete_opaque_types : & mut ConcreteOpaqueTypes < 'tcx > , opaque_types : & [(OpaqueTypeKey < 'tcx > , OpaqueHiddenType < 'tcx >)] , errors : & mut Vec < DeferredOpaqueTypeError < 'tcx > > ,) -> Vec < DefiningUse < 'tcx > > { let infcx = rcx . infcx ; let mut defining_uses = vec ! [] ; for & (opaque_type_key , hidden_type) in opaque_types { let non_nll_opaque_type_key = opaque_type_key . fold_captured_lifetime_args (infcx . tcx , | r | { nll_var_to_universal_region (& rcx , r . as_var ()) . unwrap_or (r) }) ; if let Err (err) = opaque_type_has_defining_use_args (infcx , non_nll_opaque_type_key , hidden_type . span , DefiningScopeKind :: MirBorrowck ,) { if infcx . tcx . use_typing_mode_borrowck () { match err { NonDefiningUseReason :: Tainted (guar) => add_concrete_opaque_type (infcx . tcx , concrete_opaque_types , opaque_type_key . def_id , OpaqueHiddenType :: new_error (infcx . tcx , guar) ,) , _ => debug ! (? non_nll_opaque_type_key , ? err , "ignoring non-defining use") , } } else { errors . push (DeferredOpaqueTypeError :: InvalidOpaqueTypeArgs (err)) ; debug ! ("collect_defining_uses: InvalidOpaqueTypeArgs for {:?} := {:?}" , non_nll_opaque_type_key , hidden_type) ; } continue ; } let arg_regions = iter :: once (rcx . universal_regions () . fr_static) . chain (opaque_type_key . iter_captured_args (infcx . tcx) . filter_map (| (_ , arg) | arg . as_region ()) . map (Region :: as_var) ,) . collect () ; defining_uses . push (DefiningUse { opaque_type_key : non_nll_opaque_type_key , arg_regions , hidden_type , }) ; } defining_uses }
    };
}

collect_defining_uses!()