mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_middle :: ty :: { self , CratePredicatesMap , GenericArgKind , TyCtxt , Upcast } ;}
mkuse!{use rustc_span :: Span ;}
mkmod!{dump, { 
                getname!(dump);
                getsrc!(dump);
                getpath!(dump);
                get_deps!(dump);
                get_crates!(dump);
                mkinclude!(dump);
                 
            }}
mkmod!{explicit, { 
                getname!(explicit);
                getsrc!(explicit);
                getpath!(explicit);
                get_deps!(explicit);
                get_crates!(explicit);
                mkinclude!(explicit);
                 
            }}
mkmod!{implicit_infer, { 
                getname!(implicit_infer);
                getsrc!(implicit_infer);
                getpath!(implicit_infer);
                get_deps!(implicit_infer);
                get_crates!(implicit_infer);
                mkinclude!(implicit_infer);
                 
            }}
mkmod!{utils, { 
                getname!(utils);
                getsrc!(utils);
                getpath!(utils);
                get_deps!(utils);
                get_crates!(utils);
                mkinclude!(utils);
                 
            }}

macro_rules! inferred_outlives_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inferred_outlives_of in module {}", module_path!());
    };
}

mkfn!{
    inferred_outlives_of_introspect!();
    pub (super) fn inferred_outlives_of (tcx : TyCtxt < '_ > , item_def_id : LocalDefId ,) -> & [(ty :: Clause < '_ > , Span)] { match tcx . def_kind (item_def_id) { DefKind :: Struct | DefKind :: Enum | DefKind :: Union => { let crate_map = tcx . inferred_outlives_crate (()) ; crate_map . predicates . get (& item_def_id . to_def_id ()) . copied () . unwrap_or (& []) } DefKind :: TyAlias if tcx . type_alias_is_lazy (item_def_id) => { let crate_map = tcx . inferred_outlives_crate (()) ; crate_map . predicates . get (& item_def_id . to_def_id ()) . copied () . unwrap_or (& []) } DefKind :: AnonConst if tcx . features () . generic_const_exprs () => { let id = tcx . local_def_id_to_hir_id (item_def_id) ; if tcx . hir_opt_const_param_default_param_def_id (id) . is_some () { let item_def_id = tcx . hir_get_parent_item (id) ; tcx . inferred_outlives_of (item_def_id) } else { & [] } } _ => & [] , } }
}

macro_rules! inferred_outlives_crate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inferred_outlives_crate in module {}", module_path!());
    };
}

mkfn!{
    inferred_outlives_crate_introspect!();
    pub (super) fn inferred_outlives_crate (tcx : TyCtxt < '_ > , () : ()) -> CratePredicatesMap < '_ > { let global_inferred_outlives = implicit_infer :: infer_predicates (tcx) ; let predicates = global_inferred_outlives . iter () . map (| (& def_id , set) | { let predicates = & * tcx . arena . alloc_from_iter (set . as_ref () . skip_binder () . iter () . filter_map (| (ty :: OutlivesPredicate (arg1 , region2) , & span) | { match arg1 . kind () { GenericArgKind :: Type (ty1) => Some ((ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (ty1 , * region2)) . upcast (tcx) , span ,)) , GenericArgKind :: Lifetime (region1) => Some ((ty :: ClauseKind :: RegionOutlives (ty :: OutlivesPredicate (region1 , * region2 ,)) . upcast (tcx) , span ,)) , GenericArgKind :: Const (_) => { None } } } ,)) ; (def_id , predicates) }) . collect () ; ty :: CratePredicatesMap { predicates } }
}