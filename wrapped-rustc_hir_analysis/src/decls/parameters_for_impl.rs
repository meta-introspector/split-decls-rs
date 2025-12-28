macro_rules! deps {
    () => {
        Parameter!();
    };
}

macro_rules! parameters_for_impl {
    () => {
        deps!();
        # [doc = " Returns the set of parameters constrained by the impl header."] pub (crate) fn parameters_for_impl < 'tcx > (tcx : TyCtxt < 'tcx > , impl_self_ty : Ty < 'tcx > , impl_trait_ref : Option < ty :: TraitRef < 'tcx > > ,) -> FxHashSet < Parameter > { let vec = match impl_trait_ref { Some (tr) => parameters_for (tcx , tr , false) , None => parameters_for (tcx , impl_self_ty , false) , } ; vec . into_iter () . collect () }
    };
}

parameters_for_impl!()