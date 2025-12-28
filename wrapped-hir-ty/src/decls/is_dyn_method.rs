macro_rules! deps {
    () => {
        TraitEnvironment!();
    };
}

macro_rules! is_dyn_method {
    () => {
        deps!();
        # [doc = " Checks if the self parameter of `Trait` method is the `dyn Trait` and we should"] # [doc = " call the method using the vtable."] pub fn is_dyn_method < 'db > (interner : DbInterner < 'db > , _env : Arc < TraitEnvironment < 'db > > , func : FunctionId , fn_subst : GenericArgs < 'db > ,) -> Option < usize > { let db = interner . db ; let ItemContainerId :: TraitId (trait_id) = func . loc (db) . container else { return None ; } ; let trait_params = db . generic_params (trait_id . into ()) . len () ; let fn_params = fn_subst . len () - trait_params ; let trait_ref = TraitRef :: new (interner , trait_id . into () , GenericArgs :: new_from_iter (interner , fn_subst . iter () . take (trait_params)) ,) ; let self_ty = trait_ref . self_ty () ; if let TyKind :: Dynamic (d , _) = self_ty . kind () { let is_my_trait_in_bounds = d . principal_def_id () . is_some_and (| trait_ | all_super_traits (db , trait_ . 0) . contains (& trait_id)) ; if is_my_trait_in_bounds { return Some (fn_params) ; } } None }
    };
}

is_dyn_method!()