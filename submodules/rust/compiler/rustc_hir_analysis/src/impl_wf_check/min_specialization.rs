mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_infer :: infer :: TyCtxtInferExt ;}
mkuse!{use rustc_infer :: traits :: ObligationCause ;}
mkuse!{use rustc_infer :: traits :: specialization_graph :: Node ;}
mkuse!{use rustc_middle :: ty :: trait_def :: TraitSpecializationKind ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArg , GenericArgs , GenericArgsRef , TyCtxt , TypeVisitableExt , TypingMode , } ;}
mkuse!{use rustc_span :: { ErrorGuaranteed , Span } ;}
mkuse!{use rustc_trait_selection :: error_reporting :: InferCtxtErrorExt ;}
mkuse!{use rustc_trait_selection :: traits :: { self , ObligationCtxt , translate_args_with_cause , wf } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: errors :: GenericArgsOnOverriddenImpl ;}
mkuse!{use crate :: { constrained_generic_params as cgp , errors } ;}

macro_rules! check_min_specialization_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_min_specialization in module {}", module_path!());
    };
}

mkfn!{
    check_min_specialization_introspect!();
    pub (super) fn check_min_specialization (tcx : TyCtxt < '_ > , impl_def_id : LocalDefId ,) -> Result < () , ErrorGuaranteed > { if let Some (node) = parent_specialization_node (tcx , impl_def_id) { check_always_applicable (tcx , impl_def_id , node) ? ; } Ok (()) }
}

macro_rules! parent_specialization_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parent_specialization_node in module {}", module_path!());
    };
}

mkfn!{
    parent_specialization_node_introspect!();
    fn parent_specialization_node (tcx : TyCtxt < '_ > , impl1_def_id : LocalDefId) -> Option < Node > { let trait_ref = tcx . impl_trait_ref (impl1_def_id) ? ; let trait_def = tcx . trait_def (trait_ref . skip_binder () . def_id) ; let impl2_node = trait_def . ancestors (tcx , impl1_def_id . to_def_id ()) . ok () ? . nth (1) ? ; let always_applicable_trait = matches ! (trait_def . specialization_kind , TraitSpecializationKind :: AlwaysApplicable) ; if impl2_node . is_from_trait () && ! always_applicable_trait { return None ; } if trait_def . is_marker { return None ; } Some (impl2_node) }
}

macro_rules! check_always_applicable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_always_applicable in module {}", module_path!());
    };
}

mkfn!{
    check_always_applicable_introspect!();
    # [doc = " Check that `impl1` is a sound specialization"] # [instrument (level = "debug" , skip (tcx))] fn check_always_applicable (tcx : TyCtxt < '_ > , impl1_def_id : LocalDefId , impl2_node : Node ,) -> Result < () , ErrorGuaranteed > { let span = tcx . def_span (impl1_def_id) ; let (impl1_args , impl2_args) = get_impl_args (tcx , impl1_def_id , impl2_node) ? ; let impl2_def_id = impl2_node . def_id () ; debug ! (? impl2_def_id , ? impl2_args) ; let parent_args = if impl2_node . is_from_trait () { impl2_args . to_vec () } else { unconstrained_parent_impl_args (tcx , impl2_def_id , impl2_args) } ; check_has_items (tcx , impl1_def_id , impl2_node , span) . and (check_static_lifetimes (tcx , & parent_args , span)) . and (check_duplicate_params (tcx , impl1_args , parent_args , span)) . and (check_predicates (tcx , impl1_def_id , impl1_args , impl2_node , impl2_args , span)) }
}

macro_rules! check_has_items_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_has_items in module {}", module_path!());
    };
}

mkfn!{
    check_has_items_introspect!();
    fn check_has_items (tcx : TyCtxt < '_ > , impl1_def_id : LocalDefId , impl2_node : Node , span : Span ,) -> Result < () , ErrorGuaranteed > { if let Node :: Impl (impl2_id) = impl2_node && tcx . associated_item_def_ids (impl1_def_id) . is_empty () { let base_impl_span = tcx . def_span (impl2_id) ; return Err (tcx . dcx () . emit_err (errors :: EmptySpecialization { span , base_impl_span })) ; } Ok (()) }
}

macro_rules! get_impl_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_impl_args in module {}", module_path!());
    };
}

mkfn!{
    get_impl_args_introspect!();
    # [doc = " Given a specializing impl `impl1`, and the base impl `impl2`, returns two"] # [doc = " generic parameters `(S1, S2)` that equate their trait references."] # [doc = " The returned types are expressed in terms of the generics of `impl1`."] # [doc = ""] # [doc = " Example"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " impl<A, B> Foo<A> for B { /* impl2 */ }"] # [doc = " impl<C> Foo<Vec<C>> for C { /* impl1 */ }"] # [doc = " ```"] # [doc = ""] # [doc = " Would return `S1 = [C]` and `S2 = [Vec<C>, C]`."] fn get_impl_args (tcx : TyCtxt < '_ > , impl1_def_id : LocalDefId , impl2_node : Node ,) -> Result < (GenericArgsRef < '_ > , GenericArgsRef < '_ >) , ErrorGuaranteed > { let infcx = & tcx . infer_ctxt () . build (TypingMode :: non_body_analysis ()) ; let ocx = ObligationCtxt :: new_with_diagnostics (infcx) ; let param_env = tcx . param_env (impl1_def_id) ; let impl1_span = tcx . def_span (impl1_def_id) ; let impl1_args = GenericArgs :: identity_for_item (tcx , impl1_def_id) ; let impl2_args = translate_args_with_cause (infcx , param_env , impl1_def_id . to_def_id () , impl1_args , impl2_node , & ObligationCause :: misc (impl1_span , impl1_def_id) ,) ; let errors = ocx . select_all_or_error () ; if ! errors . is_empty () { let guar = ocx . infcx . err_ctxt () . report_fulfillment_errors (errors) ; return Err (guar) ; } let assumed_wf_types = ocx . assumed_wf_types_and_report_errors (param_env , impl1_def_id) ? ; let _ = ocx . resolve_regions_and_report_errors (impl1_def_id , param_env , assumed_wf_types) ; let Ok (impl2_args) = infcx . fully_resolve (impl2_args) else { let span = tcx . def_span (impl1_def_id) ; let guar = tcx . dcx () . emit_err (GenericArgsOnOverriddenImpl { span }) ; return Err (guar) ; } ; Ok ((impl1_args , impl2_args)) }
}

macro_rules! unconstrained_parent_impl_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unconstrained_parent_impl_args in module {}", module_path!());
    };
}

mkfn!{
    unconstrained_parent_impl_args_introspect!();
    # [doc = " Returns a list of all of the unconstrained generic parameters of the given impl."] # [doc = ""] # [doc = " For example given the impl:"] # [doc = ""] # [doc = " impl<'a, T, I> ... where &'a I: IntoIterator<Item=&'a T>"] # [doc = ""] # [doc = " This would return the args corresponding to `['a, I]`, because knowing"] # [doc = " `'a` and `I` determines the value of `T`."] fn unconstrained_parent_impl_args < 'tcx > (tcx : TyCtxt < 'tcx > , impl_def_id : DefId , impl_args : GenericArgsRef < 'tcx > ,) -> Vec < GenericArg < 'tcx > > { let impl_generic_predicates = tcx . predicates_of (impl_def_id) ; let mut unconstrained_parameters = FxHashSet :: default () ; let mut constrained_params = FxHashSet :: default () ; let impl_trait_ref = tcx . impl_trait_ref (impl_def_id) . map (ty :: EarlyBinder :: instantiate_identity) ; for (clause , _) in impl_generic_predicates . predicates . iter () { if let ty :: ClauseKind :: Projection (proj) = clause . kind () . skip_binder () { let unbound_trait_ref = proj . projection_term . trait_ref (tcx) ; if Some (unbound_trait_ref) == impl_trait_ref { continue ; } unconstrained_parameters . extend (cgp :: parameters_for (tcx , proj . projection_term , true)) ; for param in cgp :: parameters_for (tcx , proj . term , false) { if ! unconstrained_parameters . contains (& param) { constrained_params . insert (param . 0) ; } } unconstrained_parameters . extend (cgp :: parameters_for (tcx , proj . term , true)) ; } } impl_args . iter () . enumerate () . filter (| & (idx , _) | ! constrained_params . contains (& (idx as u32))) . map (| (_ , arg) | arg) . collect () }
}

macro_rules! check_duplicate_params_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_duplicate_params in module {}", module_path!());
    };
}

mkfn!{
    check_duplicate_params_introspect!();
    # [doc = " Check that parameters of the derived impl don't occur more than once in the"] # [doc = " equated args of the base impl."] # [doc = ""] # [doc = " For example forbid the following:"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " impl<A> Tr for A { }"] # [doc = " impl<B> Tr for (B, B) { }"] # [doc = " ```"] # [doc = ""] # [doc = " Note that only consider the unconstrained parameters of the base impl:"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " impl<S, I: IntoIterator<Item = S>> Tr<S> for I { }"] # [doc = " impl<T> Tr<T> for Vec<T> { }"] # [doc = " ```"] # [doc = ""] # [doc = " The args for the parent impl here are `[T, Vec<T>]`, which repeats `T`,"] # [doc = " but `S` is constrained in the parent impl, so `parent_args` is only"] # [doc = " `[Vec<T>]`. This means we allow this impl."] fn check_duplicate_params < 'tcx > (tcx : TyCtxt < 'tcx > , impl1_args : GenericArgsRef < 'tcx > , parent_args : Vec < GenericArg < 'tcx > > , span : Span ,) -> Result < () , ErrorGuaranteed > { let mut base_params = cgp :: parameters_for (tcx , parent_args , true) ; base_params . sort_by_key (| param | param . 0) ; if let (_ , [duplicate , ..]) = base_params . partition_dedup () { let param = impl1_args [duplicate . 0 as usize] ; return Err (tcx . dcx () . struct_span_err (span , format ! ("specializing impl repeats parameter `{param}`")) . emit ()) ; } Ok (()) }
}

macro_rules! check_static_lifetimes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_static_lifetimes in module {}", module_path!());
    };
}

mkfn!{
    check_static_lifetimes_introspect!();
    # [doc = " Check that `'static` lifetimes are not introduced by the specializing impl."] # [doc = ""] # [doc = " For example forbid the following:"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " impl<A> Tr for A { }"] # [doc = " impl Tr for &'static i32 { }"] # [doc = " ```"] fn check_static_lifetimes < 'tcx > (tcx : TyCtxt < 'tcx > , parent_args : & Vec < GenericArg < 'tcx > > , span : Span ,) -> Result < () , ErrorGuaranteed > { if tcx . any_free_region_meets (parent_args , | r | r . is_static ()) { return Err (tcx . dcx () . emit_err (errors :: StaticSpecialize { span })) ; } Ok (()) }
}

macro_rules! check_predicates_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_predicates in module {}", module_path!());
    };
}

mkfn!{
    check_predicates_introspect!();
    # [doc = " Check whether predicates on the specializing impl (`impl1`) are allowed."] # [doc = ""] # [doc = " Each predicate `P` must be one of:"] # [doc = ""] # [doc = " * Global (not reference any parameters)."] # [doc = " * A `T: Tr` predicate where `Tr` is an always-applicable trait."] # [doc = " * Present on the base impl `impl2`."] # [doc = "     * This check is done using the `trait_predicates_eq` function below."] # [doc = " * A well-formed predicate of a type argument of the trait being implemented,"] # [doc = "   including the `Self`-type."] # [instrument (level = "debug" , skip (tcx))] fn check_predicates < 'tcx > (tcx : TyCtxt < 'tcx > , impl1_def_id : LocalDefId , impl1_args : GenericArgsRef < 'tcx > , impl2_node : Node , impl2_args : GenericArgsRef < 'tcx > , span : Span ,) -> Result < () , ErrorGuaranteed > { let impl1_predicates : Vec < _ > = traits :: elaborate (tcx , tcx . predicates_of (impl1_def_id) . instantiate (tcx , impl1_args) . into_iter () ,) . collect () ; let mut impl2_predicates = if impl2_node . is_from_trait () { Vec :: new () } else { traits :: elaborate (tcx , tcx . predicates_of (impl2_node . def_id ()) . instantiate (tcx , impl2_args) . into_iter () . map (| (c , _s) | c . as_predicate ()) ,) . collect () } ; debug ! (? impl1_predicates , ? impl2_predicates) ; let always_applicable_traits = impl1_predicates . iter () . copied () . filter (| & (clause , _span) | { matches ! (trait_specialization_kind (tcx , clause) , Some (TraitSpecializationKind :: AlwaysApplicable)) }) . map (| (c , _span) | c . as_predicate ()) ; for arg in tcx . impl_trait_ref (impl1_def_id) . unwrap () . instantiate_identity () . args { let Some (term) = arg . as_term () else { continue ; } ; let infcx = & tcx . infer_ctxt () . build (TypingMode :: non_body_analysis ()) ; let obligations = wf :: obligations (infcx , tcx . param_env (impl1_def_id) , impl1_def_id , 0 , term , span) . unwrap () ; assert ! (! obligations . has_infer ()) ; impl2_predicates . extend (traits :: elaborate (tcx , obligations) . map (| obligation | obligation . predicate)) } impl2_predicates . extend (traits :: elaborate (tcx , always_applicable_traits)) ; let mut res = Ok (()) ; for (clause , span) in impl1_predicates { if ! impl2_predicates . iter () . any (| & pred2 | clause . as_predicate () == pred2) { res = res . and (check_specialization_on (tcx , clause , span)) } } res }
}

macro_rules! check_specialization_on_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_specialization_on in module {}", module_path!());
    };
}

mkfn!{
    check_specialization_on_introspect!();
    # [instrument (level = "debug" , skip (tcx))] fn check_specialization_on < 'tcx > (tcx : TyCtxt < 'tcx > , clause : ty :: Clause < 'tcx > , span : Span ,) -> Result < () , ErrorGuaranteed > { match clause . kind () . skip_binder () { _ if clause . is_global () => Ok (()) , ty :: ClauseKind :: Trait (ty :: TraitPredicate { trait_ref , polarity : _ }) => { if matches ! (trait_specialization_kind (tcx , clause) , Some (TraitSpecializationKind :: Marker)) { Ok (()) } else { Err (tcx . dcx () . struct_span_err (span , format ! ("cannot specialize on trait `{}`" , tcx . def_path_str (trait_ref . def_id) ,) ,) . emit ()) } } ty :: ClauseKind :: Projection (ty :: ProjectionPredicate { projection_term , term }) => Err (tcx . dcx () . struct_span_err (span , format ! ("cannot specialize on associated type `{projection_term} == {term}`" ,) ,) . emit ()) , ty :: ClauseKind :: ConstArgHasType (..) => { Ok (()) } _ => Err (tcx . dcx () . struct_span_err (span , format ! ("cannot specialize on predicate `{clause}`")) . emit ()) , } }
}

macro_rules! trait_specialization_kind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trait_specialization_kind in module {}", module_path!());
    };
}

mkfn!{
    trait_specialization_kind_introspect!();
    fn trait_specialization_kind < 'tcx > (tcx : TyCtxt < 'tcx > , clause : ty :: Clause < 'tcx > ,) -> Option < TraitSpecializationKind > { match clause . kind () . skip_binder () { ty :: ClauseKind :: Trait (ty :: TraitPredicate { trait_ref , polarity : _ }) => { Some (tcx . trait_def (trait_ref . def_id) . specialization_kind) } ty :: ClauseKind :: RegionOutlives (_) | ty :: ClauseKind :: TypeOutlives (_) | ty :: ClauseKind :: Projection (_) | ty :: ClauseKind :: ConstArgHasType (..) | ty :: ClauseKind :: WellFormed (_) | ty :: ClauseKind :: ConstEvaluatable (..) | ty :: ClauseKind :: UnstableFeature (_) | ty :: ClauseKind :: HostEffect (..) => None , } }
}