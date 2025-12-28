macro_rules! dyn_trait_in_self {
    () => {
        # [doc = " This takes a valid `self` receiver type and extracts the principal trait"] # [doc = " ref of the type. Return `None` if there is no principal trait."] fn dyn_trait_in_self < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > ,) -> Option < ty :: ExistentialTraitRef < 'tcx > > { for arg in ty . peel_refs () . walk () { if let GenericArgKind :: Type (ty) = arg . kind () && let ty :: Dynamic (data , _ , _) = ty . kind () { return data . principal () . map (| principal | tcx . instantiate_bound_regions_with_erased (principal)) ; } } bug ! ("expected a `dyn Trait` ty, found {ty:?}") }
    };
}

dyn_trait_in_self!();