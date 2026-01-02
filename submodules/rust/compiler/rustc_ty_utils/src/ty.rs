mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_infer :: infer :: TyCtxtInferExt ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: { self , SizedTraitKind , Ty , TyCtxt , TypeSuperVisitable , TypeVisitable , TypeVisitor , Upcast , fold_regions , } ;}
mkuse!{use rustc_span :: DUMMY_SP ;}
mkuse!{use rustc_span :: def_id :: { CRATE_DEF_ID , DefId , LocalDefId } ;}
mkuse!{use rustc_trait_selection :: traits ;}
mkuse!{use tracing :: instrument ;}

macro_rules! sizedness_constraint_for_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sizedness_constraint_for_ty in module {}", module_path!());
    };
}

mkfn!{
    sizedness_constraint_for_ty_introspect!();
    # [doc = " If `ty` implements the given `sizedness` trait, returns `None`. Otherwise, returns the type"] # [doc = " that must implement the given `sizedness` for `ty` to implement it."] # [instrument (level = "debug" , skip (tcx) , ret)] fn sizedness_constraint_for_ty < 'tcx > (tcx : TyCtxt < 'tcx > , sizedness : SizedTraitKind , ty : Ty < 'tcx > ,) -> Option < Ty < 'tcx > > { match ty . kind () { ty :: Bool | ty :: Char | ty :: Int (..) | ty :: Uint (..) | ty :: Float (..) | ty :: RawPtr (..) | ty :: Ref (..) | ty :: FnDef (..) | ty :: FnPtr (..) | ty :: Array (..) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Coroutine (..) | ty :: CoroutineWitness (..) | ty :: Never => None , ty :: Str | ty :: Slice (..) | ty :: Dynamic (_ , _ , ty :: Dyn) => match sizedness { SizedTraitKind :: Sized => Some (ty) , SizedTraitKind :: MetaSized => None , } , ty :: Param (..) | ty :: Alias (..) | ty :: Error (_) => Some (ty) , ty :: UnsafeBinder (inner_ty) => { sizedness_constraint_for_ty (tcx , sizedness , inner_ty . skip_binder ()) . map (| _ | ty) } ty :: Foreign (..) => Some (ty) , ty :: Pat (ty , _) => sizedness_constraint_for_ty (tcx , sizedness , * ty) , ty :: Tuple (tys) => { tys . last () . and_then (| & ty | sizedness_constraint_for_ty (tcx , sizedness , ty)) } ty :: Adt (adt , args) => adt . sizedness_constraint (tcx , sizedness) . and_then (| intermediate | { let ty = intermediate . instantiate (tcx , args) ; sizedness_constraint_for_ty (tcx , sizedness , ty) }) , ty :: Placeholder (..) | ty :: Bound (..) | ty :: Infer (..) => { bug ! ("unexpected type `{ty:?}` in `sizedness_constraint_for_ty`") } } }
}

macro_rules! defaultness_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function defaultness in module {}", module_path!());
    };
}

mkfn!{
    defaultness_introspect!();
    fn defaultness (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> hir :: Defaultness { match tcx . hir_node_by_def_id (def_id) { hir :: Node :: Item (hir :: Item { kind : hir :: ItemKind :: Impl (hir :: Impl { of_trait : Some (hir :: TraitImplHeader { defaultness , .. }) , .. }) , .. }) | hir :: Node :: ImplItem (hir :: ImplItem { impl_kind : hir :: ImplItemImplKind :: Trait { defaultness , .. } , .. }) | hir :: Node :: TraitItem (hir :: TraitItem { defaultness , .. }) => * defaultness , node => { bug ! ("`defaultness` called on {:?}" , node) ; } } }
}

macro_rules! adt_sizedness_constraint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function adt_sizedness_constraint in module {}", module_path!());
    };
}

mkfn!{
    adt_sizedness_constraint_introspect!();
    # [doc = " Returns the type of the last field of a struct (\"the constraint\") which must implement the"] # [doc = " `sizedness` trait for the whole ADT to be considered to implement that `sizedness` trait."] # [doc = " `def_id` is assumed to be the `AdtDef` of a struct and will panic otherwise."] # [doc = ""] # [doc = " For `Sized`, there are only a few options for the types in the constraint:"] # [doc = "     - an meta-sized type (str, slices, trait objects, etc)"] # [doc = "     - an pointee-sized type (extern types)"] # [doc = "     - a type parameter or projection whose sizedness can't be known"] # [doc = ""] # [doc = " For `MetaSized`, there are only a few options for the types in the constraint:"] # [doc = "     - an pointee-sized type (extern types)"] # [doc = "     - a type parameter or projection whose sizedness can't be known"] # [instrument (level = "debug" , skip (tcx) , ret)] fn adt_sizedness_constraint < 'tcx > (tcx : TyCtxt < 'tcx > , (def_id , sizedness) : (DefId , SizedTraitKind) ,) -> Option < ty :: EarlyBinder < 'tcx , Ty < 'tcx > > > { if let Some (def_id) = def_id . as_local () && let ty :: Representability :: Infinite (_) = tcx . representability (def_id) { return None ; } let def = tcx . adt_def (def_id) ; if ! def . is_struct () { bug ! ("`adt_sizedness_constraint` called on non-struct type: {def:?}") ; } let tail_def = def . non_enum_variant () . tail_opt () ? ; let tail_ty = tcx . type_of (tail_def . did) . instantiate_identity () ; let constraint_ty = sizedness_constraint_for_ty (tcx , sizedness , tail_ty) ? ; let sizedness_trait_def_id = sizedness . require_lang_item (tcx) ; let predicates = tcx . predicates_of (def . did ()) . predicates ; if predicates . iter () . any (| (p , _) | { p . as_trait_clause () . is_some_and (| trait_pred | { trait_pred . def_id () == sizedness_trait_def_id && trait_pred . self_ty () . skip_binder () == constraint_ty }) }) { return None ; } Some (ty :: EarlyBinder :: bind (constraint_ty)) }
}

macro_rules! param_env_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function param_env in module {}", module_path!());
    };
}

mkfn!{
    param_env_introspect!();
    # [doc = " See `ParamEnv` struct definition for details."] fn param_env (tcx : TyCtxt < '_ > , def_id : DefId) -> ty :: ParamEnv < '_ > { let ty :: InstantiatedPredicates { mut predicates , .. } = tcx . predicates_of (def_id) . instantiate_identity (tcx) ; if tcx . def_kind (def_id) == DefKind :: AssocFn && let assoc_item = tcx . associated_item (def_id) && assoc_item . container == ty :: AssocContainer :: Trait && assoc_item . defaultness (tcx) . has_value () { let sig = tcx . fn_sig (def_id) . instantiate_identity () ; sig . skip_binder () . visit_with (& mut ImplTraitInTraitFinder { tcx , fn_def_id : def_id , bound_vars : sig . bound_vars () , predicates : & mut predicates , seen : FxHashSet :: default () , depth : ty :: INNERMOST , }) ; } if tcx . is_conditionally_const (def_id) { predicates . extend (tcx . const_conditions (def_id) . instantiate_identity (tcx) . into_iter () . map (| (trait_ref , _) | trait_ref . to_host_effect_clause (tcx , ty :: BoundConstness :: Maybe) ,) ,) ; } let local_did = def_id . as_local () ; let unnormalized_env = ty :: ParamEnv :: new (tcx . mk_clauses (& predicates)) ; let body_id = local_did . unwrap_or (CRATE_DEF_ID) ; let cause = traits :: ObligationCause :: misc (tcx . def_span (def_id) , body_id) ; traits :: normalize_param_env_or_error (tcx , unnormalized_env , cause) }
}
mkitem!{mkstruct!{# [doc = " Walk through a function type, gathering all RPITITs and installing a"] # [doc = " `NormalizesTo(Projection(RPITIT) -> Opaque(RPITIT))` predicate into the"] # [doc = " predicates list. This allows us to observe that an RPITIT projects to"] # [doc = " its corresponding opaque within the body of a default-body trait method."] struct ImplTraitInTraitFinder < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , predicates : & 'a mut Vec < ty :: Clause < 'tcx > > , fn_def_id : DefId , bound_vars : & 'tcx ty :: List < ty :: BoundVariableKind > , seen : FxHashSet < DefId > , depth : ty :: DebruijnIndex , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for ImplTraitInTraitFinder < '_ , 'tcx > { fn visit_binder < T : TypeVisitable < TyCtxt < 'tcx > > > (& mut self , binder : & ty :: Binder < 'tcx , T >) { self . depth . shift_in (1) ; binder . super_visit_with (self) ; self . depth . shift_out (1) ; } fn visit_ty (& mut self , ty : Ty < 'tcx >) { if let ty :: Alias (ty :: Projection , unshifted_alias_ty) = * ty . kind () && let Some (ty :: ImplTraitInTraitData :: Trait { fn_def_id , .. } | ty :: ImplTraitInTraitData :: Impl { fn_def_id , .. } ,) = self . tcx . opt_rpitit_info (unshifted_alias_ty . def_id) && fn_def_id == self . fn_def_id && self . seen . insert (unshifted_alias_ty . def_id) { let shifted_alias_ty = fold_regions (self . tcx , unshifted_alias_ty , | re , depth | { if let ty :: ReBound (index , bv) = re . kind () { if depth != ty :: INNERMOST { return ty :: Region :: new_error_with_message (self . tcx , DUMMY_SP , "we shouldn't walk non-predicate binders with `impl Trait`..." ,) ; } ty :: Region :: new_bound (self . tcx , index . shifted_out_to_binder (self . depth) , bv) } else { re } }) ; let default_ty = self . tcx . type_of (shifted_alias_ty . def_id) . instantiate (self . tcx , shifted_alias_ty . args) ; self . predicates . push (ty :: Binder :: bind_with_vars (ty :: ProjectionPredicate { projection_term : shifted_alias_ty . into () , term : default_ty . into () , } , self . bound_vars ,) . upcast (self . tcx) ,) ; for bound in self . tcx . item_bounds (unshifted_alias_ty . def_id) . iter_instantiated (self . tcx , unshifted_alias_ty . args) { bound . visit_with (self) ; } } ty . super_visit_with (self) } }}}

macro_rules! typing_env_normalized_for_post_analysis_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function typing_env_normalized_for_post_analysis in module {}", module_path!());
    };
}

mkfn!{
    typing_env_normalized_for_post_analysis_introspect!();
    fn typing_env_normalized_for_post_analysis (tcx : TyCtxt < '_ > , def_id : DefId) -> ty :: TypingEnv < '_ > { ty :: TypingEnv :: non_body_analysis (tcx , def_id) . with_post_analysis_normalized (tcx) }
}

macro_rules! asyncness_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function asyncness in module {}", module_path!());
    };
}

mkfn!{
    asyncness_introspect!();
    # [doc = " Check if a function is async."] fn asyncness (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> ty :: Asyncness { let node = tcx . hir_node_by_def_id (def_id) ; node . fn_sig () . map_or (ty :: Asyncness :: No , | sig | match sig . header . asyncness { hir :: IsAsync :: Async (_) => ty :: Asyncness :: Yes , hir :: IsAsync :: NotAsync => ty :: Asyncness :: No , }) }
}

macro_rules! unsizing_params_for_adt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsizing_params_for_adt in module {}", module_path!());
    };
}

mkfn!{
    unsizing_params_for_adt_introspect!();
    fn unsizing_params_for_adt < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId) -> DenseBitSet < u32 > { let def = tcx . adt_def (def_id) ; let num_params = tcx . generics_of (def_id) . count () ; let maybe_unsizing_param_idx = | arg : ty :: GenericArg < 'tcx > | match arg . kind () { ty :: GenericArgKind :: Type (ty) => match ty . kind () { ty :: Param (p) => Some (p . index) , _ => None , } , ty :: GenericArgKind :: Lifetime (_) => None , ty :: GenericArgKind :: Const (ct) => match ct . kind () { ty :: ConstKind :: Param (p) => Some (p . index) , _ => None , } , } ; let Some ((tail_field , prefix_fields)) = def . non_enum_variant () . fields . raw . split_last () else { return DenseBitSet :: new_empty (num_params) ; } ; let mut unsizing_params = DenseBitSet :: new_empty (num_params) ; for arg in tcx . type_of (tail_field . did) . instantiate_identity () . walk () { if let Some (i) = maybe_unsizing_param_idx (arg) { unsizing_params . insert (i) ; } } for field in prefix_fields { for arg in tcx . type_of (field . did) . instantiate_identity () . walk () { if let Some (i) = maybe_unsizing_param_idx (arg) { unsizing_params . remove (i) ; } } } unsizing_params }
}

macro_rules! impl_self_is_guaranteed_unsized_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function impl_self_is_guaranteed_unsized in module {}", module_path!());
    };
}

mkfn!{
    impl_self_is_guaranteed_unsized_introspect!();
    fn impl_self_is_guaranteed_unsized < 'tcx > (tcx : TyCtxt < 'tcx > , impl_def_id : DefId) -> bool { debug_assert_eq ! (tcx . def_kind (impl_def_id) , DefKind :: Impl { of_trait : true }) ; let infcx = tcx . infer_ctxt () . ignoring_regions () . build (ty :: TypingMode :: non_body_analysis ()) ; let ocx = traits :: ObligationCtxt :: new (& infcx) ; let cause = traits :: ObligationCause :: dummy () ; let param_env = tcx . param_env (impl_def_id) ; let tail = tcx . struct_tail_raw (tcx . type_of (impl_def_id) . instantiate_identity () , | ty | { ocx . structurally_normalize_ty (& cause , param_env , ty) . unwrap_or_else (| _ | { Ty :: new_error_with_message (tcx , tcx . def_span (impl_def_id) , "struct tail should be computable" ,) }) } , | | () ,) ; match tail . kind () { ty :: Dynamic (_ , _ , ty :: Dyn) | ty :: Slice (_) | ty :: Str => true , ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Adt (_ , _) | ty :: Foreign (_) | ty :: Array (_ , _) | ty :: Pat (_ , _) | ty :: RawPtr (_ , _) | ty :: Ref (_ , _ , _) | ty :: FnDef (_ , _) | ty :: FnPtr (_ , _) | ty :: UnsafeBinder (_) | ty :: Closure (_ , _) | ty :: CoroutineClosure (_ , _) | ty :: Coroutine (_ , _) | ty :: CoroutineWitness (_ , _) | ty :: Never | ty :: Tuple (_) | ty :: Alias (_ , _) | ty :: Param (_) | ty :: Bound (_ , _) | ty :: Placeholder (_) | ty :: Infer (_) | ty :: Error (_) => false , } }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { asyncness , adt_sizedness_constraint , param_env , typing_env_normalized_for_post_analysis , defaultness , unsizing_params_for_adt , impl_self_is_guaranteed_unsized , .. * providers } ; }
}