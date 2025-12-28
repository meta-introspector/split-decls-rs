macro_rules! deps {
    () => {
        FnKind!();
        InheritanceKind!();
    };
}

macro_rules! inherit_predicates_for_delegation_item {
    () => {
        deps!();
        pub (crate) fn inherit_predicates_for_delegation_item < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId , sig_id : DefId ,) -> ty :: GenericPredicates < 'tcx > { let args = create_generic_args (tcx , def_id , sig_id) ; let caller_kind = fn_kind (tcx , def_id . into ()) ; let callee_kind = fn_kind (tcx , sig_id) ; match (caller_kind , callee_kind) { (FnKind :: Free , FnKind :: Free) | (FnKind :: Free , FnKind :: AssocTrait) => { build_predicates (tcx , sig_id , None , InheritanceKind :: WithParent (true) , args) } (FnKind :: AssocTraitImpl , FnKind :: AssocTrait) => build_predicates (tcx , sig_id , Some (tcx . parent (def_id . into ())) , InheritanceKind :: Own , args ,) , (FnKind :: AssocInherentImpl , FnKind :: AssocTrait) | (FnKind :: AssocTrait , FnKind :: AssocTrait) | (FnKind :: AssocInherentImpl , FnKind :: Free) | (FnKind :: AssocTrait , FnKind :: Free) => build_predicates (tcx , sig_id , Some (tcx . parent (def_id . into ())) , InheritanceKind :: WithParent (false) , args ,) , (FnKind :: AssocTraitImpl , _) | (_ , FnKind :: AssocTraitImpl) | (_ , FnKind :: AssocInherentImpl) => unreachable ! () , } }
    };
}

inherit_predicates_for_delegation_item!();