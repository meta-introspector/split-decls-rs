mkuse!{use std :: num :: NonZero ;}
mkuse!{use rustc_ast_lowering :: stability :: extern_abi_stability ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_data_structures :: unord :: { ExtendUnord , UnordMap , UnordSet } ;}
mkuse!{use rustc_feature :: { EnabledLangFeature , EnabledLibFeature } ;}
mkuse!{use rustc_hir :: attrs :: { AttributeKind , DeprecatedSince } ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_hir :: def_id :: { CRATE_DEF_ID , LOCAL_CRATE , LocalDefId , LocalModDefId } ;}
mkuse!{use rustc_hir :: intravisit :: { self , Visitor , VisitorExt } ;}
mkuse!{use rustc_hir :: { self as hir , AmbigArg , ConstStability , DefaultBodyStability , FieldDef , Item , ItemKind , Stability , StabilityLevel , StableSince , TraitRef , Ty , TyKind , UnstableReason , VERSION_PLACEHOLDER , Variant , find_attr , } ;}
mkuse!{use rustc_middle :: hir :: nested_filter ;}
mkuse!{use rustc_middle :: middle :: lib_features :: { FeatureStability , LibFeatures } ;}
mkuse!{use rustc_middle :: middle :: privacy :: EffectiveVisibilities ;}
mkuse!{use rustc_middle :: middle :: stability :: { AllowUnstable , Deprecated , DeprecationEntry , EvalResult } ;}
mkuse!{use rustc_middle :: query :: { LocalCrate , Providers } ;}
mkuse!{use rustc_middle :: ty :: print :: with_no_trimmed_paths ;}
mkuse!{use rustc_middle :: ty :: { AssocContainer , TyCtxt } ;}
mkuse!{use rustc_session :: lint ;}
mkuse!{use rustc_session :: lint :: builtin :: { DEPRECATED , INEFFECTIVE_UNSTABLE_TRAIT_IMPL } ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use crate :: errors ;}
mkitem!{mkenum!{# [derive (PartialEq)] enum AnnotationKind { # [doc = " Annotation is required if not inherited from unstable parents."] Required , # [doc = " Annotation is useless, reject it."] Prohibited , # [doc = " Deprecation annotation is useless, reject it. (Stability attribute is still required.)"] DeprecationProhibited , # [doc = " Annotation itself is useless, but it can be propagated to children."] Container , }}}

macro_rules! inherit_deprecation_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inherit_deprecation in module {}", module_path!());
    };
}

mkfn!{
    inherit_deprecation_introspect!();
    fn inherit_deprecation (def_kind : DefKind) -> bool { match def_kind { DefKind :: LifetimeParam | DefKind :: TyParam | DefKind :: ConstParam => false , _ => true , } }
}

macro_rules! inherit_const_stability_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inherit_const_stability in module {}", module_path!());
    };
}

mkfn!{
    inherit_const_stability_introspect!();
    fn inherit_const_stability (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { let def_kind = tcx . def_kind (def_id) ; match def_kind { DefKind :: AssocFn | DefKind :: AssocTy | DefKind :: AssocConst => { match tcx . def_kind (tcx . local_parent (def_id)) { DefKind :: Impl { of_trait : true } => true , _ => false , } } _ => false , } }
}

macro_rules! annotation_kind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function annotation_kind in module {}", module_path!());
    };
}

mkfn!{
    annotation_kind_introspect!();
    fn annotation_kind (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> AnnotationKind { let def_kind = tcx . def_kind (def_id) ; match def_kind { DefKind :: Impl { of_trait : false } | DefKind :: ForeignMod => AnnotationKind :: Container , DefKind :: Impl { of_trait : true } => AnnotationKind :: DeprecationProhibited , DefKind :: TyParam | DefKind :: ConstParam => { match & tcx . hir_node_by_def_id (def_id) . expect_generic_param () . kind { hir :: GenericParamKind :: Type { default : Some (_) , .. } | hir :: GenericParamKind :: Const { default : Some (_) , .. } => { AnnotationKind :: Container } _ => AnnotationKind :: Prohibited , } } DefKind :: AssocTy | DefKind :: AssocFn | DefKind :: AssocConst => { match tcx . def_kind (tcx . local_parent (def_id)) { DefKind :: Impl { of_trait : true } => AnnotationKind :: Prohibited , _ => AnnotationKind :: Required , } } _ => AnnotationKind :: Required , } }
}

macro_rules! lookup_deprecation_entry_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lookup_deprecation_entry in module {}", module_path!());
    };
}

mkfn!{
    lookup_deprecation_entry_introspect!();
    fn lookup_deprecation_entry (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Option < DeprecationEntry > { let attrs = tcx . hir_attrs (tcx . local_def_id_to_hir_id (def_id)) ; let depr = find_attr ! (attrs , AttributeKind :: Deprecation { deprecation , span : _ } => * deprecation) ; let Some (depr) = depr else { if inherit_deprecation (tcx . def_kind (def_id)) { let parent_id = tcx . opt_local_parent (def_id) ? ; let parent_depr = tcx . lookup_deprecation_entry (parent_id) ? ; return Some (parent_depr) ; } return None ; } ; Some (DeprecationEntry :: local (depr , def_id)) }
}

macro_rules! inherit_stability_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inherit_stability in module {}", module_path!());
    };
}

mkfn!{
    inherit_stability_introspect!();
    fn inherit_stability (def_kind : DefKind) -> bool { match def_kind { DefKind :: Field | DefKind :: Variant | DefKind :: Ctor (..) => true , _ => false , } }
}
mkitem!{# [doc = " If the `-Z force-unstable-if-unmarked` flag is passed then we provide"] # [doc = " a parent stability annotation which indicates that this is private"] # [doc = " with the `rustc_private` feature. This is intended for use when"] # [doc = " compiling library and `rustc_*` crates themselves so we can leverage crates.io"] # [doc = " while maintaining the invariant that all sysroot crates are unstable"] # [doc = " by default and are unable to be used."] const FORCE_UNSTABLE : Stability = Stability { level : StabilityLevel :: Unstable { reason : UnstableReason :: Default , issue : NonZero :: new (27812) , is_soft : false , implied_by : None , old_name : None , } , feature : sym :: rustc_private , } ;}

macro_rules! lookup_stability_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lookup_stability in module {}", module_path!());
    };
}

mkfn!{
    lookup_stability_introspect!();
    # [instrument (level = "debug" , skip (tcx))] fn lookup_stability (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Option < Stability > { if ! tcx . features () . staged_api () { if ! tcx . sess . opts . unstable_opts . force_unstable_if_unmarked { return None ; } let Some (parent) = tcx . opt_local_parent (def_id) else { return Some (FORCE_UNSTABLE) } ; if inherit_deprecation (tcx . def_kind (def_id)) { let parent = tcx . lookup_stability (parent) ? ; if parent . is_unstable () { return Some (parent) ; } } return None ; } let attrs = tcx . hir_attrs (tcx . local_def_id_to_hir_id (def_id)) ; let stab = find_attr ! (attrs , AttributeKind :: Stability { stability , span : _ } => * stability) ; if let Some (stab) = stab { return Some (stab) ; } if inherit_deprecation (tcx . def_kind (def_id)) { let Some (parent) = tcx . opt_local_parent (def_id) else { return tcx . sess . opts . unstable_opts . force_unstable_if_unmarked . then_some (FORCE_UNSTABLE) ; } ; let parent = tcx . lookup_stability (parent) ? ; if parent . is_unstable () || inherit_stability (tcx . def_kind (def_id)) { return Some (parent) ; } } None }
}

macro_rules! lookup_default_body_stability_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lookup_default_body_stability in module {}", module_path!());
    };
}

mkfn!{
    lookup_default_body_stability_introspect!();
    # [instrument (level = "debug" , skip (tcx))] fn lookup_default_body_stability (tcx : TyCtxt < '_ > , def_id : LocalDefId ,) -> Option < DefaultBodyStability > { if ! tcx . features () . staged_api () { return None ; } let attrs = tcx . hir_attrs (tcx . local_def_id_to_hir_id (def_id)) ; find_attr ! (attrs , AttributeKind :: BodyStability { stability , .. } => * stability) }
}

macro_rules! lookup_const_stability_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lookup_const_stability in module {}", module_path!());
    };
}

mkfn!{
    lookup_const_stability_introspect!();
    # [instrument (level = "debug" , skip (tcx))] fn lookup_const_stability (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Option < ConstStability > { if ! tcx . features () . staged_api () { if inherit_deprecation (tcx . def_kind (def_id)) { let parent = tcx . opt_local_parent (def_id) ? ; let parent_stab = tcx . lookup_stability (parent) ? ; if parent_stab . is_unstable () && let Some (fn_sig) = tcx . hir_node_by_def_id (def_id) . fn_sig () && fn_sig . header . is_const () { let attrs = tcx . hir_attrs (tcx . local_def_id_to_hir_id (def_id)) ; let const_stability_indirect = find_attr ! (attrs , AttributeKind :: ConstStabilityIndirect) ; return Some (ConstStability :: unmarked (const_stability_indirect , parent_stab)) ; } } return None ; } let attrs = tcx . hir_attrs (tcx . local_def_id_to_hir_id (def_id)) ; let const_stability_indirect = find_attr ! (attrs , AttributeKind :: ConstStabilityIndirect) ; let const_stab = find_attr ! (attrs , AttributeKind :: ConstStability { stability , span : _ } => * stability) ; let mut const_stab = const_stab . map (| const_stab | ConstStability :: from_partial (const_stab , const_stability_indirect)) ; if let Some (fn_sig) = tcx . hir_node_by_def_id (def_id) . fn_sig () && fn_sig . header . is_const () && const_stab . is_none () && let Some (inherit_regular_stab) = tcx . lookup_stability (def_id) && inherit_regular_stab . is_unstable () { const_stab = Some (ConstStability { const_stable_indirect : true , promotable : false , level : inherit_regular_stab . level , feature : inherit_regular_stab . feature , }) ; } if let Some (const_stab) = const_stab { return Some (const_stab) ; } if inherit_const_stability (tcx , def_id) { let parent = tcx . opt_local_parent (def_id) ? ; let parent = tcx . lookup_const_stability (parent) ? ; if parent . is_const_unstable () { return Some (parent) ; } } None }
}

macro_rules! stability_implications_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stability_implications in module {}", module_path!());
    };
}

mkfn!{
    stability_implications_introspect!();
    fn stability_implications (tcx : TyCtxt < '_ > , LocalCrate : LocalCrate) -> UnordMap < Symbol , Symbol > { let mut implications = UnordMap :: default () ; let mut register_implication = | def_id | { if let Some (stability) = tcx . lookup_stability (def_id) && let StabilityLevel :: Unstable { implied_by : Some (implied_by) , .. } = stability . level { implications . insert (implied_by , stability . feature) ; } if let Some (stability) = tcx . lookup_const_stability (def_id) && let StabilityLevel :: Unstable { implied_by : Some (implied_by) , .. } = stability . level { implications . insert (implied_by , stability . feature) ; } } ; if tcx . features () . staged_api () { register_implication (CRATE_DEF_ID) ; for def_id in tcx . hir_crate_items (()) . definitions () { register_implication (def_id) ; let def_kind = tcx . def_kind (def_id) ; if def_kind . is_adt () { let adt = tcx . adt_def (def_id) ; for variant in adt . variants () { if variant . def_id != def_id . to_def_id () { register_implication (variant . def_id . expect_local ()) ; } for field in & variant . fields { register_implication (field . did . expect_local ()) ; } if let Some (ctor_def_id) = variant . ctor_def_id () { register_implication (ctor_def_id . expect_local ()) } } } if def_kind . has_generics () { for param in tcx . generics_of (def_id) . own_params . iter () { register_implication (param . def_id . expect_local ()) } } } } implications }
}
mkitem!{mkstruct!{struct MissingStabilityAnnotations < 'tcx > { tcx : TyCtxt < 'tcx > , effective_visibilities : & 'tcx EffectiveVisibilities , }}}
mkitem!{mkimpl!{impl < 'tcx > MissingStabilityAnnotations < 'tcx > { # [doc = " Verify that deprecation and stability attributes make sense with one another."] # [instrument (level = "trace" , skip (self))] fn check_compatible_stability (& self , def_id : LocalDefId) { if ! self . tcx . features () . staged_api () { return ; } let depr = self . tcx . lookup_deprecation_entry (def_id) ; let stab = self . tcx . lookup_stability (def_id) ; let const_stab = self . tcx . lookup_const_stability (def_id) ; macro_rules ! find_attr_span { ($ name : ident) => { { let attrs = self . tcx . hir_attrs (self . tcx . local_def_id_to_hir_id (def_id)) ; find_attr ! (attrs , AttributeKind ::$ name { span , .. } => * span) } } } if stab . is_none () && depr . map_or (false , | d | d . attr . is_since_rustc_version ()) && let Some (span) = find_attr_span ! (Deprecation) { self . tcx . dcx () . emit_err (errors :: DeprecatedAttribute { span }) ; } if let Some (stab) = stab { let kind = annotation_kind (self . tcx , def_id) ; if kind == AnnotationKind :: Prohibited || (kind == AnnotationKind :: Container && stab . level . is_stable () && depr . is_some ()) { if let Some (span) = find_attr_span ! (Stability) { let item_sp = self . tcx . def_span (def_id) ; self . tcx . dcx () . emit_err (errors :: UselessStability { span , item_sp }) ; } } if let Some (depr) = depr && let DeprecatedSince :: RustcVersion (dep_since) = depr . attr . since && let StabilityLevel :: Stable { since : stab_since , .. } = stab . level && let Some (span) = find_attr_span ! (Stability) { let item_sp = self . tcx . def_span (def_id) ; match stab_since { StableSince :: Current => { self . tcx . dcx () . emit_err (errors :: CannotStabilizeDeprecated { span , item_sp }) ; } StableSince :: Version (stab_since) => { if dep_since < stab_since { self . tcx . dcx () . emit_err (errors :: CannotStabilizeDeprecated { span , item_sp }) ; } } StableSince :: Err (_) => { } } } } let fn_sig = self . tcx . hir_node_by_def_id (def_id) . fn_sig () ; if let Some (fn_sig) = fn_sig && ! fn_sig . header . is_const () && const_stab . is_some () && find_attr_span ! (ConstStability) . is_some () { self . tcx . dcx () . emit_err (errors :: MissingConstErr { fn_sig_span : fn_sig . span }) ; } if let Some (const_stab) = const_stab && let Some (fn_sig) = fn_sig && const_stab . is_const_stable () && ! stab . is_some_and (| s | s . is_stable ()) && let Some (const_span) = find_attr_span ! (ConstStability) { self . tcx . dcx () . emit_err (errors :: ConstStableNotStable { fn_sig_span : fn_sig . span , const_span }) ; } if let Some (stab) = & const_stab && stab . is_const_stable () && stab . const_stable_indirect && let Some (span) = find_attr_span ! (ConstStability) { self . tcx . dcx () . emit_err (errors :: RustcConstStableIndirectPairing { span }) ; } } # [instrument (level = "debug" , skip (self))] fn check_missing_stability (& self , def_id : LocalDefId) { let stab = self . tcx . lookup_stability (def_id) ; self . tcx . ensure_ok () . lookup_const_stability (def_id) ; if ! self . tcx . sess . is_test_crate () && stab . is_none () && self . effective_visibilities . is_reachable (def_id) { let descr = self . tcx . def_descr (def_id . to_def_id ()) ; let span = self . tcx . def_span (def_id) ; self . tcx . dcx () . emit_err (errors :: MissingStabilityAttr { span , descr }) ; } } fn check_missing_const_stability (& self , def_id : LocalDefId) { let is_const = self . tcx . is_const_fn (def_id . to_def_id ()) || (self . tcx . def_kind (def_id . to_def_id ()) == DefKind :: Trait && self . tcx . is_const_trait (def_id . to_def_id ())) ; if is_const && self . effective_visibilities . is_reachable (def_id) && self . tcx . lookup_const_stability (def_id) . is_none () { let span = self . tcx . def_span (def_id) ; let descr = self . tcx . def_descr (def_id . to_def_id ()) ; self . tcx . dcx () . emit_err (errors :: MissingConstStabAttr { span , descr }) ; } } }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for MissingStabilityAnnotations < 'tcx > { type NestedFilter = nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_item (& mut self , i : & 'tcx Item < 'tcx >) { self . check_compatible_stability (i . owner_id . def_id) ; if ! matches ! (i . kind , hir :: ItemKind :: Impl (hir :: Impl { of_trait : None , .. }) | hir :: ItemKind :: ForeignMod { .. }) { self . check_missing_stability (i . owner_id . def_id) ; } self . check_missing_const_stability (i . owner_id . def_id) ; intravisit :: walk_item (self , i) } fn visit_trait_item (& mut self , ti : & 'tcx hir :: TraitItem < 'tcx >) { self . check_compatible_stability (ti . owner_id . def_id) ; self . check_missing_stability (ti . owner_id . def_id) ; intravisit :: walk_trait_item (self , ti) ; } fn visit_impl_item (& mut self , ii : & 'tcx hir :: ImplItem < 'tcx >) { self . check_compatible_stability (ii . owner_id . def_id) ; if let hir :: ImplItemImplKind :: Inherent { .. } = ii . impl_kind { self . check_missing_stability (ii . owner_id . def_id) ; self . check_missing_const_stability (ii . owner_id . def_id) ; } intravisit :: walk_impl_item (self , ii) ; } fn visit_variant (& mut self , var : & 'tcx Variant < 'tcx >) { self . check_compatible_stability (var . def_id) ; self . check_missing_stability (var . def_id) ; if let Some (ctor_def_id) = var . data . ctor_def_id () { self . check_missing_stability (ctor_def_id) ; } intravisit :: walk_variant (self , var) ; } fn visit_field_def (& mut self , s : & 'tcx FieldDef < 'tcx >) { self . check_compatible_stability (s . def_id) ; self . check_missing_stability (s . def_id) ; intravisit :: walk_field_def (self , s) ; } fn visit_foreign_item (& mut self , i : & 'tcx hir :: ForeignItem < 'tcx >) { self . check_compatible_stability (i . owner_id . def_id) ; self . check_missing_stability (i . owner_id . def_id) ; intravisit :: walk_foreign_item (self , i) ; } fn visit_generic_param (& mut self , p : & 'tcx hir :: GenericParam < 'tcx >) { self . check_compatible_stability (p . def_id) ; intravisit :: walk_generic_param (self , p) ; } }}}

macro_rules! check_mod_unstable_api_usage_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_mod_unstable_api_usage in module {}", module_path!());
    };
}

mkfn!{
    check_mod_unstable_api_usage_introspect!();
    # [doc = " Cross-references the feature names of unstable APIs with enabled"] # [doc = " features and possibly prints errors."] fn check_mod_unstable_api_usage (tcx : TyCtxt < '_ > , module_def_id : LocalModDefId) { tcx . hir_visit_item_likes_in_module (module_def_id , & mut Checker { tcx }) ; let is_staged_api = tcx . sess . opts . unstable_opts . force_unstable_if_unmarked || tcx . features () . staged_api () ; if is_staged_api { let effective_visibilities = & tcx . effective_visibilities (()) ; let mut missing = MissingStabilityAnnotations { tcx , effective_visibilities } ; if module_def_id . is_top_level_module () { missing . check_missing_stability (CRATE_DEF_ID) ; } tcx . hir_visit_item_likes_in_module (module_def_id , & mut missing) ; } if module_def_id . is_top_level_module () { check_unused_or_stable_features (tcx) } }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { check_mod_unstable_api_usage , stability_implications , lookup_stability , lookup_const_stability , lookup_default_body_stability , lookup_deprecation_entry , .. * providers } ; }
}
mkitem!{mkstruct!{struct Checker < 'tcx > { tcx : TyCtxt < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for Checker < 'tcx > { type NestedFilter = nested_filter :: OnlyBodies ; # [doc = " Because stability levels are scoped lexically, we want to walk"] # [doc = " nested items in the context of the outer item, so enable"] # [doc = " deep-walking."] fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_item (& mut self , item : & 'tcx hir :: Item < 'tcx >) { match item . kind { hir :: ItemKind :: ExternCrate (_ , ident) => { if item . span . is_dummy () && ident . name != sym :: std { return ; } let Some (cnum) = self . tcx . extern_mod_stmt_cnum (item . owner_id . def_id) else { return ; } ; let def_id = cnum . as_def_id () ; self . tcx . check_stability (def_id , Some (item . hir_id ()) , item . span , None) ; } hir :: ItemKind :: Impl (hir :: Impl { of_trait : Some (of_trait) , self_ty , items , .. }) => { let features = self . tcx . features () ; if features . staged_api () { let attrs = self . tcx . hir_attrs (item . hir_id ()) ; let stab = find_attr ! (attrs , AttributeKind :: Stability { stability , span } => (* stability , * span)) ; let const_stab = find_attr ! (attrs , AttributeKind :: ConstStability { stability , .. } => * stability) ; let unstable_feature_stab = find_attr ! (attrs , AttributeKind :: UnstableFeatureBound (i) => i) . map (| i | i . as_slice ()) . unwrap_or_default () ; if let Some ((Stability { level : StabilityLevel :: Unstable { .. } , feature } , span ,)) = stab { let mut c = CheckTraitImplStable { tcx : self . tcx , fully_stable : true } ; c . visit_ty_unambig (self_ty) ; c . visit_trait_ref (& of_trait . trait_ref) ; let mut unstable_feature_bound_in_effect = false ; for (unstable_bound_feat_name , _) in unstable_feature_stab { if * unstable_bound_feat_name == feature { unstable_feature_bound_in_effect = true ; } } if of_trait . trait_ref . path . res != Res :: Err && c . fully_stable && ! unstable_feature_bound_in_effect { self . tcx . emit_node_span_lint (INEFFECTIVE_UNSTABLE_TRAIT_IMPL , item . hir_id () , span , errors :: IneffectiveUnstableImpl ,) ; } } if features . const_trait_impl () && let hir :: Constness :: Const = of_trait . constness { let stable_or_implied_stable = match const_stab { None => true , Some (stab) if stab . is_const_stable () => { self . tcx . dcx () . emit_err (errors :: TraitImplConstStable { span : item . span }) ; true } Some (_) => false , } ; if let Some (trait_id) = of_trait . trait_ref . trait_def_id () && let Some (const_stab) = self . tcx . lookup_const_stability (trait_id) { if const_stab . is_const_stable () != stable_or_implied_stable { let trait_span = self . tcx . def_ident_span (trait_id) . unwrap () ; let impl_stability = if stable_or_implied_stable { errors :: ImplConstStability :: Stable { span : item . span } } else { errors :: ImplConstStability :: Unstable { span : item . span } } ; let trait_stability = if const_stab . is_const_stable () { errors :: TraitConstStability :: Stable { span : trait_span } } else { errors :: TraitConstStability :: Unstable { span : trait_span } } ; self . tcx . dcx () . emit_err (errors :: TraitImplConstStabilityMismatch { span : item . span , impl_stability , trait_stability , }) ; } } } } if let hir :: Constness :: Const = of_trait . constness && let Some (def_id) = of_trait . trait_ref . trait_def_id () { self . tcx . check_const_stability (def_id , of_trait . trait_ref . path . span , of_trait . trait_ref . path . span ,) ; } for impl_item_ref in items { let impl_item = self . tcx . associated_item (impl_item_ref . owner_id) ; if let AssocContainer :: TraitImpl (Ok (def_id)) = impl_item . container { self . tcx . check_stability (def_id , None , self . tcx . def_span (impl_item_ref . owner_id) , None ,) ; } } } _ => () , } intravisit :: walk_item (self , item) ; } fn visit_poly_trait_ref (& mut self , t : & 'tcx hir :: PolyTraitRef < 'tcx >) { match t . modifiers . constness { hir :: BoundConstness :: Always (span) | hir :: BoundConstness :: Maybe (span) => { if let Some (def_id) = t . trait_ref . trait_def_id () { self . tcx . check_const_stability (def_id , t . trait_ref . path . span , span) ; } } hir :: BoundConstness :: Never => { } } intravisit :: walk_poly_trait_ref (self , t) ; } fn visit_path (& mut self , path : & hir :: Path < 'tcx > , id : hir :: HirId) { if let Some (def_id) = path . res . opt_def_id () { let method_span = path . segments . last () . map (| s | s . ident . span) ; let item_is_allowed = self . tcx . check_stability_allow_unstable (def_id , Some (id) , path . span , method_span , if is_unstable_reexport (self . tcx , id) { AllowUnstable :: Yes } else { AllowUnstable :: No } ,) ; if item_is_allowed { let is_allowed_through_unstable_modules : Option < Symbol > = self . tcx . lookup_stability (def_id) . and_then (| stab | match stab . level { StabilityLevel :: Stable { allowed_through_unstable_modules , .. } => { allowed_through_unstable_modules } _ => None , }) ; let parents = path . segments . iter () . rev () . skip (1) ; for path_segment in parents { if let Some (def_id) = path_segment . res . opt_def_id () { match is_allowed_through_unstable_modules { None => { self . tcx . check_stability_allow_unstable (def_id , None , path . span , None , if is_unstable_reexport (self . tcx , id) { AllowUnstable :: Yes } else { AllowUnstable :: No } ,) ; } Some (deprecation) => { let eval_result = self . tcx . eval_stability_allow_unstable (def_id , None , path . span , None , if is_unstable_reexport (self . tcx , id) { AllowUnstable :: Yes } else { AllowUnstable :: No } ,) ; let is_allowed = matches ! (eval_result , EvalResult :: Allow) ; if ! is_allowed { if self . tcx . lint_level_at_node (DEPRECATED , id) . level == lint :: Level :: Allow { return ; } let def_path = with_no_trimmed_paths ! (self . tcx . def_path_str (def_id)) ; let def_kind = self . tcx . def_descr (def_id) ; let diag = Deprecated { sub : None , kind : def_kind . to_owned () , path : def_path , note : Some (deprecation) , since_kind : lint :: DeprecatedSinceKind :: InEffect , } ; self . tcx . emit_node_span_lint (DEPRECATED , id , method_span . unwrap_or (path . span) , diag ,) ; } } } } } } } intravisit :: walk_path (self , path) } }}}

macro_rules! is_unstable_reexport_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_unstable_reexport in module {}", module_path!());
    };
}

mkfn!{
    is_unstable_reexport_introspect!();
    # [doc = " Check whether a path is a `use` item that has been marked as unstable."] # [doc = ""] # [doc = " See issue #94972 for details on why this is a special case"] fn is_unstable_reexport (tcx : TyCtxt < '_ > , id : hir :: HirId) -> bool { let Some (owner) = id . as_owner () else { return false ; } ; let def_id = owner . def_id ; let Some (stab) = tcx . lookup_stability (def_id) else { return false ; } ; if stab . level . is_stable () { return false ; } if ! matches ! (tcx . hir_expect_item (def_id) . kind , ItemKind :: Use (..)) { return false ; } true }
}
mkitem!{mkstruct!{struct CheckTraitImplStable < 'tcx > { tcx : TyCtxt < 'tcx > , fully_stable : bool , }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for CheckTraitImplStable < 'tcx > { fn visit_path (& mut self , path : & hir :: Path < 'tcx > , _id : hir :: HirId) { if let Some (def_id) = path . res . opt_def_id () && let Some (stab) = self . tcx . lookup_stability (def_id) { self . fully_stable &= stab . level . is_stable () ; } intravisit :: walk_path (self , path) } fn visit_trait_ref (& mut self , t : & 'tcx TraitRef < 'tcx >) { if let Res :: Def (DefKind :: Trait , trait_did) = t . path . res { if let Some (stab) = self . tcx . lookup_stability (trait_did) { self . fully_stable &= stab . level . is_stable () ; } } intravisit :: walk_trait_ref (self , t) } fn visit_ty (& mut self , t : & 'tcx Ty < 'tcx , AmbigArg >) { if let TyKind :: Never = t . kind { self . fully_stable = false ; } if let TyKind :: FnPtr (function) = t . kind { if extern_abi_stability (function . abi) . is_err () { self . fully_stable = false ; } } intravisit :: walk_ty (self , t) } fn visit_fn_decl (& mut self , fd : & 'tcx hir :: FnDecl < 'tcx >) { for ty in fd . inputs { self . visit_ty_unambig (ty) } if let hir :: FnRetTy :: Return (output_ty) = fd . output { match output_ty . kind { TyKind :: Never => { } _ => self . visit_ty_unambig (output_ty) , } } } }}}

macro_rules! check_unused_or_stable_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_unused_or_stable_features in module {}", module_path!());
    };
}

mkfn!{
    check_unused_or_stable_features_introspect!();
    # [doc = " Given the list of enabled features that were not language features (i.e., that"] # [doc = " were expected to be library features), and the list of features used from"] # [doc = " libraries, identify activated features that don't exist and error about them."] pub fn check_unused_or_stable_features (tcx : TyCtxt < '_ >) { let _prof_timer = tcx . sess . timer ("unused_lib_feature_checking") ; let enabled_lang_features = tcx . features () . enabled_lang_features () ; let mut lang_features = UnordSet :: default () ; for EnabledLangFeature { gate_name , attr_sp , stable_since } in enabled_lang_features { if let Some (version) = stable_since { unnecessary_stable_feature_lint (tcx , * attr_sp , * gate_name , * version) ; } if ! lang_features . insert (gate_name) { tcx . dcx () . emit_err (errors :: DuplicateFeatureErr { span : * attr_sp , feature : * gate_name }) ; } } let enabled_lib_features = tcx . features () . enabled_lib_features () ; let mut remaining_lib_features = FxIndexMap :: default () ; for EnabledLibFeature { gate_name , attr_sp } in enabled_lib_features { if remaining_lib_features . contains_key (gate_name) { tcx . dcx () . emit_err (errors :: DuplicateFeatureErr { span : * attr_sp , feature : * gate_name }) ; } remaining_lib_features . insert (* gate_name , * attr_sp) ; } remaining_lib_features . swap_remove (& sym :: libc) ; remaining_lib_features . swap_remove (& sym :: test) ; # [doc = " For each feature in `defined_features`.."] # [doc = ""] # [doc = " - If it is in `remaining_lib_features` (those features with `#![feature(..)]` attributes in"] # [doc = "   the current crate), check if it is stable (or partially stable) and thus an unnecessary"] # [doc = "   attribute."] # [doc = " - If it is in `remaining_implications` (a feature that is referenced by an `implied_by`"] # [doc = "   from the current crate), then remove it from the remaining implications."] # [doc = ""] # [doc = " Once this function has been invoked for every feature (local crate and all extern crates),"] # [doc = " then.."] # [doc = ""] # [doc = " - If features remain in `remaining_lib_features`, then the user has enabled a feature that"] # [doc = "   does not exist."] # [doc = " - If features remain in `remaining_implications`, the `implied_by` refers to a feature that"] # [doc = "   does not exist."] # [doc = ""] # [doc = " By structuring the code in this way: checking the features defined from each crate one at a"] # [doc = " time, less loading from metadata is performed and thus compiler performance is improved."] fn check_features < 'tcx > (tcx : TyCtxt < 'tcx > , remaining_lib_features : & mut FxIndexMap < Symbol , Span > , remaining_implications : & mut UnordMap < Symbol , Symbol > , defined_features : & LibFeatures , all_implications : & UnordMap < Symbol , Symbol > ,) { for (feature , stability) in defined_features . to_sorted_vec () { if let FeatureStability :: AcceptedSince (since) = stability && let Some (span) = remaining_lib_features . get (& feature) { if let Some (implies) = all_implications . get (& feature) { unnecessary_partially_stable_feature_lint (tcx , * span , feature , * implies , since) ; } else { unnecessary_stable_feature_lint (tcx , * span , feature , since) ; } } remaining_lib_features . swap_remove (& feature) ; remaining_implications . remove (& feature) ; if let FeatureStability :: Unstable { old_name : Some (alias) } = stability && let Some (span) = remaining_lib_features . swap_remove (& alias) { tcx . dcx () . emit_err (errors :: RenamedFeature { span , feature , alias }) ; } if remaining_lib_features . is_empty () && remaining_implications . is_empty () { break ; } } } let mut remaining_implications = tcx . stability_implications (LOCAL_CRATE) . clone () ; let local_defined_features = tcx . lib_features (LOCAL_CRATE) ; if ! remaining_lib_features . is_empty () || ! remaining_implications . is_empty () { let mut all_implications = remaining_implications . clone () ; for & cnum in tcx . crates (()) { all_implications . extend_unord (tcx . stability_implications (cnum) . items () . map (| (k , v) | (* k , * v))) ; } check_features (tcx , & mut remaining_lib_features , & mut remaining_implications , local_defined_features , & all_implications ,) ; for & cnum in tcx . crates (()) { if remaining_lib_features . is_empty () && remaining_implications . is_empty () { break ; } check_features (tcx , & mut remaining_lib_features , & mut remaining_implications , tcx . lib_features (cnum) , & all_implications ,) ; } } for (feature , span) in remaining_lib_features { tcx . dcx () . emit_err (errors :: UnknownFeature { span , feature }) ; } for (& implied_by , & feature) in remaining_implications . to_sorted_stable_ord () { let local_defined_features = tcx . lib_features (LOCAL_CRATE) ; let span = local_defined_features . stability . get (& feature) . expect ("feature that implied another does not exist") . 1 ; tcx . dcx () . emit_err (errors :: ImpliedFeatureNotExist { span , feature , implied_by }) ; } }
}

macro_rules! unnecessary_partially_stable_feature_lint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unnecessary_partially_stable_feature_lint in module {}", module_path!());
    };
}

mkfn!{
    unnecessary_partially_stable_feature_lint_introspect!();
    fn unnecessary_partially_stable_feature_lint (tcx : TyCtxt < '_ > , span : Span , feature : Symbol , implies : Symbol , since : Symbol ,) { tcx . emit_node_span_lint (lint :: builtin :: STABLE_FEATURES , hir :: CRATE_HIR_ID , span , errors :: UnnecessaryPartialStableFeature { span , line : tcx . sess . source_map () . span_extend_to_line (span) , feature , since , implies , } ,) ; }
}

macro_rules! unnecessary_stable_feature_lint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unnecessary_stable_feature_lint in module {}", module_path!());
    };
}

mkfn!{
    unnecessary_stable_feature_lint_introspect!();
    fn unnecessary_stable_feature_lint (tcx : TyCtxt < '_ > , span : Span , feature : Symbol , mut since : Symbol ,) { if since . as_str () == VERSION_PLACEHOLDER { since = sym :: env_CFG_RELEASE ; } tcx . emit_node_span_lint (lint :: builtin :: STABLE_FEATURES , hir :: CRATE_HIR_ID , span , errors :: UnnecessaryStableFeature { feature , since } ,) ; }
}