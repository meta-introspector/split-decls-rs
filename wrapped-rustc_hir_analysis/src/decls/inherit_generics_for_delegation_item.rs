macro_rules! deps {
    () => {
        FnKind!();
        InheritanceKind!();
    };
}

macro_rules! inherit_generics_for_delegation_item {
    () => {
        deps!();
        pub (crate) fn inherit_generics_for_delegation_item < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId , sig_id : DefId ,) -> ty :: Generics { let caller_kind = fn_kind (tcx , def_id . into ()) ; let callee_kind = fn_kind (tcx , sig_id) ; match (caller_kind , callee_kind) { (FnKind :: Free , FnKind :: Free) | (FnKind :: Free , FnKind :: AssocTrait) => { build_generics (tcx , sig_id , None , InheritanceKind :: WithParent (true)) } (FnKind :: AssocTraitImpl , FnKind :: AssocTrait) => { build_generics (tcx , sig_id , Some (tcx . parent (def_id . into ())) , InheritanceKind :: Own) } (FnKind :: AssocInherentImpl , FnKind :: AssocTrait) | (FnKind :: AssocTrait , FnKind :: AssocTrait) | (FnKind :: AssocInherentImpl , FnKind :: Free) | (FnKind :: AssocTrait , FnKind :: Free) => build_generics (tcx , sig_id , Some (tcx . parent (def_id . into ())) , InheritanceKind :: WithParent (false) ,) , (FnKind :: AssocTraitImpl , _) | (_ , FnKind :: AssocTraitImpl) | (_ , FnKind :: AssocInherentImpl) => unreachable ! () , } }
    };
}

inherit_generics_for_delegation_item!()