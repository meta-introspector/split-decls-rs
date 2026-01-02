mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}

macro_rules! parent_impl_or_trait_constness_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parent_impl_or_trait_constness in module {}", module_path!());
    };
}

mkfn!{
    parent_impl_or_trait_constness_introspect!();
    fn parent_impl_or_trait_constness (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> hir :: Constness { let parent_id = tcx . local_parent (def_id) ; match tcx . def_kind (parent_id) { DefKind :: Impl { of_trait : true } => tcx . impl_trait_header (parent_id) . unwrap () . constness , DefKind :: Trait => { if tcx . is_const_trait (parent_id . into ()) { hir :: Constness :: Const } else { hir :: Constness :: NotConst } } _ => hir :: Constness :: NotConst , } }
}

macro_rules! constness_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function constness in module {}", module_path!());
    };
}

mkfn!{
    constness_introspect!();
    # [doc = " Checks whether a function-like definition is considered to be `const`."] fn constness (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> hir :: Constness { let node = tcx . hir_node_by_def_id (def_id) ; match node { hir :: Node :: Ctor (hir :: VariantData :: Tuple (..)) => hir :: Constness :: Const , hir :: Node :: ForeignItem (item) if let hir :: ForeignItemKind :: Fn (..) = item . kind => { hir :: Constness :: NotConst } hir :: Node :: Expr (e) if let hir :: ExprKind :: Closure (c) = e . kind => c . constness , _ => { if let Some (fn_kind) = node . fn_kind () { if fn_kind . constness () == hir :: Constness :: Const { return hir :: Constness :: Const ; } parent_impl_or_trait_constness (tcx , def_id) } else { tcx . dcx () . span_bug (tcx . def_span (def_id) , format ! ("should not be requesting the constness of items that can't be const: {node:#?}: {:?}" , tcx . def_kind (def_id))) } } } }
}

macro_rules! is_promotable_const_fn_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_promotable_const_fn in module {}", module_path!());
    };
}

mkfn!{
    is_promotable_const_fn_introspect!();
    fn is_promotable_const_fn (tcx : TyCtxt < '_ > , def_id : DefId) -> bool { tcx . is_const_fn (def_id) && match tcx . lookup_const_stability (def_id) { Some (stab) => { if cfg ! (debug_assertions) && stab . promotable { let sig = tcx . fn_sig (def_id) ; assert ! (sig . skip_binder () . safety () . is_safe () , "don't mark const unsafe fns as promotable" ,) ; } stab . promotable } None => false , } }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (providers : & mut Providers) { * providers = Providers { constness , is_promotable_const_fn , .. * providers } ; }
}