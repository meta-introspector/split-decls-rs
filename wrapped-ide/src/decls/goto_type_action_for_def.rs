macro_rules! deps {
    () => {
        HoverAction!();
    };
}

macro_rules! goto_type_action_for_def {
    () => {
        deps!();
        fn goto_type_action_for_def (sema : & Semantics < '_ , RootDatabase > , def : Definition , notable_traits : & [(hir :: Trait , Vec < (Option < hir :: Type < '_ > > , hir :: Name) >)] , subst_types : Option < Vec < (hir :: Symbol , hir :: Type < '_ >) > > , edition : Edition ,) -> Option < HoverAction > { let db = sema . db ; let mut targets : Vec < hir :: ModuleDef > = Vec :: new () ; let mut push_new_def = | item : hir :: ModuleDef | { if ! targets . contains (& item) { targets . push (item) ; } } ; for & (trait_ , ref assocs) in notable_traits { push_new_def (trait_ . into ()) ; assocs . iter () . filter_map (| (ty , _) | ty . as_ref ()) . for_each (| ty | { walk_and_push_ty (db , ty , & mut push_new_def) ; }) ; } if let Ok (generic_def) = GenericDef :: try_from (def) { generic_def . type_or_const_params (db) . into_iter () . for_each (| it | { walk_and_push_ty (db , & it . ty (db) , & mut push_new_def) ; }) ; } let ty = match def { Definition :: Local (it) => Some (it . ty (db)) , Definition :: Field (field) => Some (field . ty (db) . to_type (db)) , Definition :: TupleField (field) => Some (field . ty (db)) , Definition :: Const (it) => Some (it . ty (db)) , Definition :: Static (it) => Some (it . ty (db)) , Definition :: Function (func) => { for param in func . assoc_fn_params (db) { walk_and_push_ty (db , param . ty () , & mut push_new_def) ; } Some (func . ret_type (db)) } Definition :: GenericParam (hir :: GenericParam :: ConstParam (it)) => Some (it . ty (db)) , Definition :: GenericParam (hir :: GenericParam :: TypeParam (it)) => Some (it . ty (db)) , _ => None , } ; if let Some (ty) = ty { walk_and_push_ty (db , & ty , & mut push_new_def) ; } if let Some (subst_types) = subst_types { for (_ , ty) in subst_types { walk_and_push_ty (db , & ty , & mut push_new_def) ; } } HoverAction :: goto_type_from_targets (sema , targets , edition) }
    };
}

goto_type_action_for_def!()