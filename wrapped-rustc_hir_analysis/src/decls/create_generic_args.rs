macro_rules! deps {
    () => {
        FnKind!();
    };
}

macro_rules! create_generic_args {
    () => {
        deps!();
        fn create_generic_args < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId , sig_id : DefId ,) -> ty :: GenericArgsRef < 'tcx > { let caller_kind = fn_kind (tcx , def_id . into ()) ; let callee_kind = fn_kind (tcx , sig_id) ; match (caller_kind , callee_kind) { (FnKind :: Free , FnKind :: Free) | (FnKind :: Free , FnKind :: AssocTrait) | (FnKind :: AssocInherentImpl , FnKind :: Free) | (FnKind :: AssocTrait , FnKind :: Free) | (FnKind :: AssocTrait , FnKind :: AssocTrait) => { let args = ty :: GenericArgs :: identity_for_item (tcx , sig_id) ; build_generic_args (tcx , sig_id , def_id , args) } (FnKind :: AssocTraitImpl , FnKind :: AssocTrait) => { let callee_generics = tcx . generics_of (sig_id) ; let parent = tcx . parent (def_id . into ()) ; let parent_args = tcx . impl_trait_header (parent) . unwrap () . trait_ref . instantiate_identity () . args ; let trait_args = ty :: GenericArgs :: identity_for_item (tcx , sig_id) ; let method_args = tcx . mk_args (& trait_args [callee_generics . parent_count ..]) ; let method_args = build_generic_args (tcx , sig_id , def_id , method_args) ; tcx . mk_args_from_iter (parent_args . iter () . chain (method_args)) } (FnKind :: AssocInherentImpl , FnKind :: AssocTrait) => { let parent = tcx . parent (def_id . into ()) ; let self_ty = tcx . type_of (parent) . instantiate_identity () ; let generic_self_ty = ty :: GenericArg :: from (self_ty) ; let trait_args = ty :: GenericArgs :: identity_for_item (tcx , sig_id) ; let trait_args = build_generic_args (tcx , sig_id , def_id , trait_args) ; let args = std :: iter :: once (generic_self_ty) . chain (trait_args . iter () . skip (1)) ; tcx . mk_args_from_iter (args) } (FnKind :: AssocTraitImpl , _) | (_ , FnKind :: AssocTraitImpl) | (_ , FnKind :: AssocInherentImpl) => unreachable ! () , } }
    };
}

create_generic_args!()