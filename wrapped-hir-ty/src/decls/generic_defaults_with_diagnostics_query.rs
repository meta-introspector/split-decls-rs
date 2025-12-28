macro_rules! deps {
    () => {
        GenericDefaults!();
        EarlyBinder!();
        LifetimeElisionKind!();
        TyLoweringContext!();
        ImplTraitLoweringMode!();
        HirDatabase!();
        Diagnostics!();
    };
}

macro_rules! generic_defaults_with_diagnostics_query {
    () => {
        deps!();
        # [doc = " Resolve the default type params from generics."] # [doc = ""] # [doc = " Diagnostics are only returned for this `GenericDefId` (returned defaults include parents)."] pub (crate) fn generic_defaults_with_diagnostics_query (db : & dyn HirDatabase , def : GenericDefId ,) -> (GenericDefaults < '_ > , Diagnostics) { let generic_params = generics (db , def) ; if generic_params . is_empty () { return (GenericDefaults (None) , None) ; } let resolver = def . resolver (db) ; let mut ctx = TyLoweringContext :: new (db , & resolver , generic_params . store () , def , LifetimeElisionKind :: AnonymousReportError ,) . with_impl_trait_mode (ImplTraitLoweringMode :: Disallowed) ; let mut idx = 0 ; let mut has_any_default = false ; let mut defaults = generic_params . iter_parents_with_store () . map (| ((_id , p) , store) | { ctx . store = store ; let (result , has_default) = handle_generic_param (& mut ctx , idx , p) ; has_any_default |= has_default ; idx += 1 ; result }) . collect :: < Vec < _ > > () ; ctx . diagnostics . clear () ; defaults . extend (generic_params . iter_self () . map (| (_id , p) | { let (result , has_default) = handle_generic_param (& mut ctx , idx , p) ; has_any_default |= has_default ; idx += 1 ; result })) ; let diagnostics = create_diagnostics (mem :: take (& mut ctx . diagnostics)) ; let defaults = if has_any_default { GenericDefaults (Some (Arc :: from_iter (defaults))) } else { GenericDefaults (None) } ; return (defaults , diagnostics) ; fn handle_generic_param < 'db > (ctx : & mut TyLoweringContext < 'db , '_ > , idx : usize , p : GenericParamDataRef < '_ > ,) -> (Option < EarlyBinder < 'db , GenericArg < 'db > > > , bool) { ctx . lowering_param_default (idx as u32) ; match p { GenericParamDataRef :: TypeParamData (p) => { let ty = p . default . map (| ty | ctx . lower_ty (ty)) ; (ty . map (| ty | EarlyBinder :: bind (ty . into ())) , p . default . is_some ()) } GenericParamDataRef :: ConstParamData (p) => { let val = p . default . map (| c | { let param_ty = ctx . lower_ty (p . ty) ; let c = ctx . lower_const (c , param_ty) ; c . into () }) ; (val . map (EarlyBinder :: bind) , p . default . is_some ()) } GenericParamDataRef :: LifetimeParamData (_) => (None , false) , } } }
    };
}

generic_defaults_with_diagnostics_query!()