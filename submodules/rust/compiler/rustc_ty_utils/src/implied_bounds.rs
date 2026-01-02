mkuse!{use std :: iter ;}
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , fold_regions } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: Span ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { assumed_wf_types , assumed_wf_types_for_rpitit : | tcx , def_id | { assert ! (tcx . is_impl_trait_in_trait (def_id . to_def_id ())) ; tcx . assumed_wf_types (def_id) } , .. * providers } ; }
}

macro_rules! assumed_wf_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assumed_wf_types in module {}", module_path!());
    };
}

mkfn!{
    assumed_wf_types_introspect!();
    fn assumed_wf_types < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId) -> & 'tcx [(Ty < 'tcx > , Span)] { let kind = tcx . def_kind (def_id) ; match kind { DefKind :: Fn => { let sig = tcx . fn_sig (def_id) . instantiate_identity () ; let liberated_sig = tcx . liberate_late_bound_regions (def_id . to_def_id () , sig) ; tcx . arena . alloc_from_iter (itertools :: zip_eq (liberated_sig . inputs_and_output , fn_sig_spans (tcx , def_id) ,)) } DefKind :: AssocFn => { let sig = tcx . fn_sig (def_id) . instantiate_identity () ; let liberated_sig = tcx . liberate_late_bound_regions (def_id . to_def_id () , sig) ; let mut assumed_wf_types : Vec < _ > = tcx . assumed_wf_types (tcx . local_parent (def_id)) . into () ; assumed_wf_types . extend (itertools :: zip_eq (liberated_sig . inputs_and_output , fn_sig_spans (tcx , def_id) ,)) ; tcx . arena . alloc_slice (& assumed_wf_types) } DefKind :: Impl { .. } => { let tys = match tcx . impl_trait_ref (def_id) { Some (trait_ref) => trait_ref . skip_binder () . args . types () . collect () , None => vec ! [tcx . type_of (def_id) . instantiate_identity ()] , } ; let mut impl_spans = impl_spans (tcx , def_id) ; tcx . arena . alloc_from_iter (tys . into_iter () . map (| ty | (ty , impl_spans . next () . unwrap ()))) } DefKind :: AssocTy if let Some (data) = tcx . opt_rpitit_info (def_id . to_def_id ()) => { match data { ty :: ImplTraitInTraitData :: Trait { fn_def_id , .. } => { let mut mapping = FxHashMap :: default () ; let generics = tcx . generics_of (def_id) ; for param in & generics . own_params [tcx . generics_of (fn_def_id) . own_params . len () ..] { let orig_lt = tcx . map_opaque_lifetime_to_parent_lifetime (param . def_id . expect_local ()) ; if matches ! (orig_lt . kind () , ty :: ReLateParam (..)) { mapping . insert (orig_lt , ty :: Region :: new_early_param (tcx , ty :: EarlyParamRegion { index : param . index , name : param . name } ,) ,) ; } } let remapped_wf_tys = fold_regions (tcx , tcx . assumed_wf_types (fn_def_id . expect_local ()) . to_vec () , | region , _ | { if let Some (remapped_region) = mapping . get (& region) { * remapped_region } else { region } } ,) ; tcx . arena . alloc_from_iter (remapped_wf_tys) } ty :: ImplTraitInTraitData :: Impl { .. } => { let impl_def_id = tcx . local_parent (def_id) ; let rpitit_def_id = tcx . trait_item_of (def_id) . unwrap () ; let args = ty :: GenericArgs :: identity_for_item (tcx , def_id) . rebase_onto (tcx , impl_def_id . to_def_id () , tcx . impl_trait_ref (impl_def_id) . unwrap () . instantiate_identity () . args ,) ; tcx . arena . alloc_from_iter (ty :: EarlyBinder :: bind (tcx . assumed_wf_types_for_rpitit (rpitit_def_id)) . iter_instantiated_copied (tcx , args) . chain (tcx . assumed_wf_types (impl_def_id) . into_iter () . copied ()) ,) } } } DefKind :: AssocConst | DefKind :: AssocTy => tcx . assumed_wf_types (tcx . local_parent (def_id)) , DefKind :: Static { .. } | DefKind :: Const | DefKind :: AnonConst | DefKind :: InlineConst | DefKind :: Struct | DefKind :: Union | DefKind :: Enum | DefKind :: Trait | DefKind :: TraitAlias | DefKind :: TyAlias => ty :: List :: empty () , DefKind :: OpaqueTy | DefKind :: Mod | DefKind :: Variant | DefKind :: ForeignTy | DefKind :: TyParam | DefKind :: ConstParam | DefKind :: Ctor (_ , _) | DefKind :: Macro (_) | DefKind :: ExternCrate | DefKind :: Use | DefKind :: ForeignMod | DefKind :: Field | DefKind :: LifetimeParam | DefKind :: GlobalAsm | DefKind :: Closure | DefKind :: SyntheticCoroutineBody => { span_bug ! (tcx . def_span (def_id) , "`assumed_wf_types` not defined for {} `{def_id:?}`" , kind . descr (def_id . to_def_id ())) ; } } }
}

macro_rules! fn_sig_spans_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fn_sig_spans in module {}", module_path!());
    };
}

mkfn!{
    fn_sig_spans_introspect!();
    fn fn_sig_spans (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> impl Iterator < Item = Span > { let node = tcx . hir_node_by_def_id (def_id) ; if let Some (decl) = node . fn_decl () { decl . inputs . iter () . map (| ty | ty . span) . chain (iter :: once (decl . output . span ())) } else { bug ! ("unexpected item for fn {def_id:?}: {node:?}") } }
}

macro_rules! impl_spans_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function impl_spans in module {}", module_path!());
    };
}

mkfn!{
    impl_spans_introspect!();
    fn impl_spans (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> impl Iterator < Item = Span > { let item = tcx . hir_expect_item (def_id) ; if let hir :: ItemKind :: Impl (impl_) = item . kind { let trait_args = impl_ . of_trait . into_iter () . flat_map (| of_trait | of_trait . trait_ref . path . segments . last () . unwrap () . args () . args) . map (| arg | arg . span ()) ; let dummy_spans_for_default_args = impl_ . of_trait . into_iter () . flat_map (| of_trait | iter :: repeat (of_trait . trait_ref . path . span)) ; iter :: once (impl_ . self_ty . span) . chain (trait_args) . chain (dummy_spans_for_default_args) } else { bug ! ("unexpected item for impl {def_id:?}: {item:?}") } }
}