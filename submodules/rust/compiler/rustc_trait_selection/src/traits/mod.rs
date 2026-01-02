mkmod!{auto_trait, { 
                getname!(auto_trait);
                getsrc!(auto_trait);
                getpath!(auto_trait);
                get_deps!(auto_trait);
                get_crates!(auto_trait);
                mkinclude!(auto_trait);
                 
            }}
mkmod!{coherence, { 
                getname!(coherence);
                getsrc!(coherence);
                getpath!(coherence);
                get_deps!(coherence);
                get_crates!(coherence);
                mkinclude!(coherence);
                 
            }}
mkmod!{const_evaluatable, { 
                getname!(const_evaluatable);
                getsrc!(const_evaluatable);
                getpath!(const_evaluatable);
                get_deps!(const_evaluatable);
                get_crates!(const_evaluatable);
                mkinclude!(const_evaluatable);
                 
            }}
mkmod!{dyn_compatibility, { 
                getname!(dyn_compatibility);
                getsrc!(dyn_compatibility);
                getpath!(dyn_compatibility);
                get_deps!(dyn_compatibility);
                get_crates!(dyn_compatibility);
                mkinclude!(dyn_compatibility);
                 
            }}
mkmod!{effects, { 
                getname!(effects);
                getsrc!(effects);
                getpath!(effects);
                get_deps!(effects);
                get_crates!(effects);
                mkinclude!(effects);
                 
            }}
mkmod!{engine, { 
                getname!(engine);
                getsrc!(engine);
                getpath!(engine);
                get_deps!(engine);
                get_crates!(engine);
                mkinclude!(engine);
                 
            }}
mkmod!{fulfill, { 
                getname!(fulfill);
                getsrc!(fulfill);
                getpath!(fulfill);
                get_deps!(fulfill);
                get_crates!(fulfill);
                mkinclude!(fulfill);
                 
            }}
mkmod!{misc, { 
                getname!(misc);
                getsrc!(misc);
                getpath!(misc);
                get_deps!(misc);
                get_crates!(misc);
                mkinclude!(misc);
                 
            }}
mkmod!{normalize, { 
                getname!(normalize);
                getsrc!(normalize);
                getpath!(normalize);
                get_deps!(normalize);
                get_crates!(normalize);
                mkinclude!(normalize);
                 
            }}
mkmod!{outlives_bounds, { 
                getname!(outlives_bounds);
                getsrc!(outlives_bounds);
                getpath!(outlives_bounds);
                get_deps!(outlives_bounds);
                get_crates!(outlives_bounds);
                mkinclude!(outlives_bounds);
                 
            }}
mkmod!{project, { 
                getname!(project);
                getsrc!(project);
                getpath!(project);
                get_deps!(project);
                get_crates!(project);
                mkinclude!(project);
                 
            }}
mkmod!{query, { 
                getname!(query);
                getsrc!(query);
                getpath!(query);
                get_deps!(query);
                get_crates!(query);
                mkinclude!(query);
                 
            }}
mkmod!{select, { 
                getname!(select);
                getsrc!(select);
                getpath!(select);
                get_deps!(select);
                get_crates!(select);
                mkinclude!(select);
                 
            }}
mkmod!{specialize, { 
                getname!(specialize);
                getsrc!(specialize);
                getpath!(specialize);
                get_deps!(specialize);
                get_crates!(specialize);
                mkinclude!(specialize);
                 
            }}
mkmod!{structural_normalize, { 
                getname!(structural_normalize);
                getsrc!(structural_normalize);
                getpath!(structural_normalize);
                get_deps!(structural_normalize);
                get_crates!(structural_normalize);
                mkinclude!(structural_normalize);
                 
            }}
mkmod!{util, { 
                getname!(util);
                getsrc!(util);
                getpath!(util);
                get_deps!(util);
                get_crates!(util);
                mkinclude!(util);
                 
            }}
mkmod!{vtable, { 
                getname!(vtable);
                getsrc!(vtable);
                getpath!(vtable);
                get_deps!(vtable);
                get_crates!(vtable);
                mkinclude!(vtable);
                 
            }}
mkmod!{wf, { 
                getname!(wf);
                getsrc!(wf);
                getpath!(wf);
                get_deps!(wf);
                get_crates!(wf);
                mkinclude!(wf);
                 
            }}
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{pub use rustc_infer :: traits :: * ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: ty :: error :: { ExpectedFound , TypeError } ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgs , GenericArgsRef , Ty , TyCtxt , TypeFoldable , TypeFolder , TypeSuperFoldable , TypeSuperVisitable , TypeVisitable , TypeVisitableExt , TypingMode , Upcast , } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{pub use self :: coherence :: { InCrate , IsFirstInputType , OrphanCheckErr , OrphanCheckMode , OverlapResult , UncoveredTyParams , add_placeholder_note , orphan_check_trait_ref , overlapping_impls , } ;}
mkuse!{pub use self :: dyn_compatibility :: { DynCompatibilityViolation , dyn_compatibility_violations_for_assoc_item , hir_ty_lowering_dyn_compatibility_violations , is_vtable_safe_method , } ;}
mkuse!{pub use self :: engine :: { ObligationCtxt , TraitEngineExt } ;}
mkuse!{pub use self :: fulfill :: { FulfillmentContext , OldSolverError , PendingPredicateObligation } ;}
mkuse!{pub use self :: normalize :: NormalizeExt ;}
mkuse!{pub use self :: project :: { normalize_inherent_projection , normalize_projection_term } ;}
mkuse!{pub use self :: select :: { EvaluationCache , EvaluationResult , IntercrateAmbiguityCause , OverflowError , SelectionCache , SelectionContext , } ;}
mkuse!{pub use self :: specialize :: specialization_graph :: { FutureCompatOverlapError , FutureCompatOverlapErrorKind , } ;}
mkuse!{pub use self :: specialize :: { OverlapError , specialization_graph , translate_args , translate_args_with_cause , } ;}
mkuse!{pub use self :: structural_normalize :: StructurallyNormalizeExt ;}
mkuse!{pub use self :: util :: { BoundVarReplacer , PlaceholderReplacer , elaborate , expand_trait_aliases , impl_item_is_final , sizedness_fast_path , supertrait_def_ids , supertraits , transitive_bounds_that_define_assoc_item , upcast_choices , with_replaced_escaping_bound_vars , } ;}
mkuse!{use crate :: error_reporting :: InferCtxtErrorExt ;}
mkuse!{use crate :: infer :: outlives :: env :: OutlivesEnvironment ;}
mkuse!{use crate :: infer :: { InferCtxt , TyCtxtInferExt } ;}
mkuse!{use crate :: regions :: InferCtxtRegionExt ;}
mkuse!{use crate :: traits :: query :: evaluate_obligation :: InferCtxtExt as _ ;}
mkitem!{mkstruct!{# [derive (Debug)] pub struct FulfillmentError < 'tcx > { pub obligation : PredicateObligation < 'tcx > , pub code : FulfillmentErrorCode < 'tcx > , # [doc = " Diagnostics only: the 'root' obligation which resulted in"] # [doc = " the failure to process `obligation`. This is the obligation"] # [doc = " that was initially passed to `register_predicate_obligation`"] pub root_obligation : PredicateObligation < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > FulfillmentError < 'tcx > { pub fn new (obligation : PredicateObligation < 'tcx > , code : FulfillmentErrorCode < 'tcx > , root_obligation : PredicateObligation < 'tcx > ,) -> FulfillmentError < 'tcx > { FulfillmentError { obligation , code , root_obligation } } pub fn is_true_error (& self) -> bool { match self . code { FulfillmentErrorCode :: Select (_) | FulfillmentErrorCode :: Project (_) | FulfillmentErrorCode :: Subtype (_ , _) | FulfillmentErrorCode :: ConstEquate (_ , _) => true , FulfillmentErrorCode :: Cycle (_) | FulfillmentErrorCode :: Ambiguity { overflow : _ } => { false } } } }}}
mkitem!{mkenum!{# [derive (Clone)] pub enum FulfillmentErrorCode < 'tcx > { # [doc = " Inherently impossible to fulfill; this trait is implemented if and only"] # [doc = " if it is already implemented."] Cycle (PredicateObligations < 'tcx >) , Select (SelectionError < 'tcx >) , Project (MismatchedProjectionTypes < 'tcx >) , Subtype (ExpectedFound < Ty < 'tcx > > , TypeError < 'tcx >) , ConstEquate (ExpectedFound < ty :: Const < 'tcx > > , TypeError < 'tcx >) , Ambiguity { # [doc = " Overflow is only `Some(suggest_recursion_limit)` when using the next generation"] # [doc = " trait solver `-Znext-solver`. With the old solver overflow is eagerly handled by"] # [doc = " emitting a fatal error instead."] overflow : Option < bool > , } , }}}
mkitem!{mkimpl!{impl < 'tcx > Debug for FulfillmentErrorCode < 'tcx > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match * self { FulfillmentErrorCode :: Select (ref e) => write ! (f , "{e:?}") , FulfillmentErrorCode :: Project (ref e) => write ! (f , "{e:?}") , FulfillmentErrorCode :: Subtype (ref a , ref b) => { write ! (f , "CodeSubtypeError({a:?}, {b:?})") } FulfillmentErrorCode :: ConstEquate (ref a , ref b) => { write ! (f , "CodeConstEquateError({a:?}, {b:?})") } FulfillmentErrorCode :: Ambiguity { overflow : None } => write ! (f , "Ambiguity") , FulfillmentErrorCode :: Ambiguity { overflow : Some (suggest_increasing_limit) } => { write ! (f , "Overflow({suggest_increasing_limit})") } FulfillmentErrorCode :: Cycle (ref cycle) => write ! (f , "Cycle({cycle:?})") , } } }}}
mkitem!{mkenum!{# [doc = " Whether to skip the leak check, as part of a future compatibility warning step."] # [doc = ""] # [doc = " The \"default\" for skip-leak-check corresponds to the current"] # [doc = " behavior (do not skip the leak check) -- not the behavior we are"] # [doc = " transitioning into."] # [derive (Copy , Clone , PartialEq , Eq , Debug , Default)] pub enum SkipLeakCheck { Yes , # [default] No , }}}
mkitem!{mkimpl!{impl SkipLeakCheck { fn is_yes (self) -> bool { self == SkipLeakCheck :: Yes } }}}
mkitem!{mkenum!{# [doc = " The mode that trait queries run in."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum TraitQueryMode { # [doc = " Standard/un-canonicalized queries get accurate"] # [doc = " spans etc. passed in and hence can do reasonable"] # [doc = " error reporting on their own."] Standard , # [doc = " Canonical queries get dummy spans and hence"] # [doc = " must generally propagate errors to"] # [doc = " pre-canonicalization callsites."] Canonical , }}}

macro_rules! predicates_for_generics_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function predicates_for_generics in module {}", module_path!());
    };
}

mkfn!{
    predicates_for_generics_introspect!();
    # [doc = " Creates predicate obligations from the generic bounds."] # [instrument (level = "debug" , skip (cause , param_env))] pub fn predicates_for_generics < 'tcx > (cause : impl Fn (usize , Span) -> ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , generic_bounds : ty :: InstantiatedPredicates < 'tcx > ,) -> impl Iterator < Item = PredicateObligation < 'tcx > > { generic_bounds . into_iter () . enumerate () . map (move | (idx , (clause , span)) | Obligation { cause : cause (idx , span) , recursion_depth : 0 , param_env , predicate : clause . as_predicate () , }) }
}

macro_rules! type_known_to_meet_bound_modulo_regions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_known_to_meet_bound_modulo_regions in module {}", module_path!());
    };
}

mkfn!{
    type_known_to_meet_bound_modulo_regions_introspect!();
    # [doc = " Determines whether the type `ty` is known to meet `bound` and"] # [doc = " returns true if so. Returns false if `ty` either does not meet"] # [doc = " `bound` or is not known to meet bound (note that this is"] # [doc = " conservative towards *no impl*, which is the opposite of the"] # [doc = " `evaluate` methods)."] pub fn type_known_to_meet_bound_modulo_regions < 'tcx > (infcx : & InferCtxt < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , ty : Ty < 'tcx > , def_id : DefId ,) -> bool { let trait_ref = ty :: TraitRef :: new (infcx . tcx , def_id , [ty]) ; pred_known_to_hold_modulo_regions (infcx , param_env , trait_ref) }
}

macro_rules! pred_known_to_hold_modulo_regions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pred_known_to_hold_modulo_regions in module {}", module_path!());
    };
}

mkfn!{
    pred_known_to_hold_modulo_regions_introspect!();
    # [doc = " FIXME(@lcnr): this function doesn't seem right and shouldn't exist?"] # [doc = ""] # [doc = " Ping me on zulip if you want to use this method and need help with finding"] # [doc = " an appropriate replacement."] # [instrument (level = "debug" , skip (infcx , param_env , pred) , ret)] fn pred_known_to_hold_modulo_regions < 'tcx > (infcx : & InferCtxt < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , pred : impl Upcast < TyCtxt < 'tcx > , ty :: Predicate < 'tcx > > ,) -> bool { let obligation = Obligation :: new (infcx . tcx , ObligationCause :: dummy () , param_env , pred) ; let result = infcx . evaluate_obligation_no_overflow (& obligation) ; debug ! (? result) ; if result . must_apply_modulo_regions () { true } else if result . may_apply () && ! infcx . next_trait_solver () { let goal = infcx . resolve_vars_if_possible ((obligation . predicate , obligation . param_env)) ; infcx . probe (| _ | { let ocx = ObligationCtxt :: new (infcx) ; ocx . register_obligation (obligation) ; let errors = ocx . select_all_or_error () ; match errors . as_slice () { [] => infcx . resolve_vars_if_possible (goal) == goal , errors => { debug ! (? errors) ; false } } }) } else { false } }
}

macro_rules! do_normalize_predicates_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function do_normalize_predicates in module {}", module_path!());
    };
}

mkfn!{
    do_normalize_predicates_introspect!();
    # [instrument (level = "debug" , skip (tcx , elaborated_env))] fn do_normalize_predicates < 'tcx > (tcx : TyCtxt < 'tcx > , cause : ObligationCause < 'tcx > , elaborated_env : ty :: ParamEnv < 'tcx > , predicates : Vec < ty :: Clause < 'tcx > > ,) -> Result < Vec < ty :: Clause < 'tcx > > , ErrorGuaranteed > { let span = cause . span ; let infcx = tcx . infer_ctxt () . ignoring_regions () . build (TypingMode :: non_body_analysis ()) ; let ocx = ObligationCtxt :: new_with_diagnostics (& infcx) ; let predicates = ocx . normalize (& cause , elaborated_env , predicates) ; let errors = ocx . select_all_or_error () ; if ! errors . is_empty () { let reported = infcx . err_ctxt () . report_fulfillment_errors (errors) ; return Err (reported) ; } debug ! ("do_normalize_predicates: normalized predicates = {:?}" , predicates) ; let errors = infcx . resolve_regions (cause . body_id , elaborated_env , []) ; if ! errors . is_empty () { tcx . dcx () . span_delayed_bug (span , format ! ("failed region resolution while normalizing {elaborated_env:?}: {errors:?}") ,) ; } match infcx . fully_resolve (predicates) { Ok (predicates) => Ok (predicates) , Err (fixup_err) => { span_bug ! (span , "inference variables in normalized parameter environment: {}" , fixup_err) ; } } }
}

macro_rules! normalize_param_env_or_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function normalize_param_env_or_error in module {}", module_path!());
    };
}

mkfn!{
    normalize_param_env_or_error_introspect!();
    # [doc = " Normalizes the parameter environment, reporting errors if they occur."] # [instrument (level = "debug" , skip (tcx))] pub fn normalize_param_env_or_error < 'tcx > (tcx : TyCtxt < 'tcx > , unnormalized_env : ty :: ParamEnv < 'tcx > , cause : ObligationCause < 'tcx > ,) -> ty :: ParamEnv < 'tcx > { let mut predicates : Vec < _ > = util :: elaborate (tcx , unnormalized_env . caller_bounds () . into_iter () . map (| predicate | { if tcx . features () . generic_const_exprs () || tcx . next_trait_solver_globally () { return predicate ; } struct ConstNormalizer < 'tcx > (TyCtxt < 'tcx >) ; impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for ConstNormalizer < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . 0 } fn fold_const (& mut self , c : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { if c . has_escaping_bound_vars () { return ty :: Const :: new_misc_error (self . 0) ; } if let ty :: ConstKind :: Unevaluated (uv) = c . kind () && self . 0 . def_kind (uv . def) == DefKind :: AnonConst { let infcx = self . 0 . infer_ctxt () . build (TypingMode :: non_body_analysis ()) ; let c = evaluate_const (& infcx , c , ty :: ParamEnv :: empty ()) ; assert ! (! c . has_infer () && ! c . has_placeholders ()) ; return c ; } c } } predicate . fold_with (& mut ConstNormalizer (tcx)) }) ,) . collect () ; debug ! ("normalize_param_env_or_error: elaborated-predicates={:?}" , predicates) ; let elaborated_env = ty :: ParamEnv :: new (tcx . mk_clauses (& predicates)) ; if ! elaborated_env . has_aliases () { return elaborated_env ; } let outlives_predicates : Vec < _ > = predicates . extract_if (.. , | predicate | { matches ! (predicate . kind () . skip_binder () , ty :: ClauseKind :: TypeOutlives (..)) }) . collect () ; debug ! ("normalize_param_env_or_error: predicates=(non-outlives={:?}, outlives={:?})" , predicates , outlives_predicates) ; let Ok (non_outlives_predicates) = do_normalize_predicates (tcx , cause . clone () , elaborated_env , predicates) else { debug ! ("normalize_param_env_or_error: errored resolving non-outlives predicates") ; return elaborated_env ; } ; debug ! ("normalize_param_env_or_error: non-outlives predicates={:?}" , non_outlives_predicates) ; let outlives_env = non_outlives_predicates . iter () . chain (& outlives_predicates) . cloned () ; let outlives_env = ty :: ParamEnv :: new (tcx . mk_clauses_from_iter (outlives_env)) ; let Ok (outlives_predicates) = do_normalize_predicates (tcx , cause , outlives_env , outlives_predicates) else { debug ! ("normalize_param_env_or_error: errored resolving outlives predicates") ; return elaborated_env ; } ; debug ! ("normalize_param_env_or_error: outlives predicates={:?}" , outlives_predicates) ; let mut predicates = non_outlives_predicates ; predicates . extend (outlives_predicates) ; debug ! ("normalize_param_env_or_error: final predicates={:?}" , predicates) ; ty :: ParamEnv :: new (tcx . mk_clauses (& predicates)) }
}
mkitem!{mkenum!{# [derive (Debug)] pub enum EvaluateConstErr { # [doc = " The constant being evaluated was either a generic parameter or inference variable, *or*,"] # [doc = " some unevaluated constant with either generic parameters or inference variables in its"] # [doc = " generic arguments."] HasGenericsOrInfers , # [doc = " The type this constant evaluated to is not valid for use in const generics. This should"] # [doc = " always result in an error when checking the constant is correctly typed for the parameter"] # [doc = " it is an argument to, so a bug is delayed when encountering this."] InvalidConstParamTy (ErrorGuaranteed) , # [doc = " CTFE failed to evaluate the constant in some unrecoverable way (e.g. encountered a `panic!`)."] # [doc = " This is also used when the constant was already tainted by error."] EvaluationFailure (ErrorGuaranteed) , }}}

macro_rules! evaluate_const_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_const in module {}", module_path!());
    };
}

mkfn!{
    evaluate_const_introspect!();
    # [doc = " Evaluates a type system constant returning a `ConstKind::Error` in cases where CTFE failed and"] # [doc = " returning the passed in constant if it was not fully concrete (i.e. depended on generic parameters"] # [doc = " or inference variables)"] # [doc = ""] # [doc = " You should not call this function unless you are implementing normalization itself. Prefer to use"] # [doc = " `normalize_erasing_regions` or the `normalize` functions on `ObligationCtxt`/`FnCtxt`/`InferCtxt`."] pub fn evaluate_const < 'tcx > (infcx : & InferCtxt < 'tcx > , ct : ty :: Const < 'tcx > , param_env : ty :: ParamEnv < 'tcx > ,) -> ty :: Const < 'tcx > { match try_evaluate_const (infcx , ct , param_env) { Ok (ct) => ct , Err (EvaluateConstErr :: EvaluationFailure (e) | EvaluateConstErr :: InvalidConstParamTy (e)) => { ty :: Const :: new_error (infcx . tcx , e) } Err (EvaluateConstErr :: HasGenericsOrInfers) => ct , } }
}

macro_rules! try_evaluate_const_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_evaluate_const in module {}", module_path!());
    };
}

mkfn!{
    try_evaluate_const_introspect!();
    # [doc = " Evaluates a type system constant making sure to not allow constants that depend on generic parameters"] # [doc = " or inference variables to succeed in evaluating."] # [doc = ""] # [doc = " You should not call this function unless you are implementing normalization itself. Prefer to use"] # [doc = " `normalize_erasing_regions` or the `normalize` functions on `ObligationCtxt`/`FnCtxt`/`InferCtxt`."] # [instrument (level = "debug" , skip (infcx) , ret)] pub fn try_evaluate_const < 'tcx > (infcx : & InferCtxt < 'tcx > , ct : ty :: Const < 'tcx > , param_env : ty :: ParamEnv < 'tcx > ,) -> Result < ty :: Const < 'tcx > , EvaluateConstErr > { let tcx = infcx . tcx ; let ct = infcx . resolve_vars_if_possible (ct) ; debug ! (? ct) ; match ct . kind () { ty :: ConstKind :: Value (..) => Ok (ct) , ty :: ConstKind :: Error (e) => Err (EvaluateConstErr :: EvaluationFailure (e)) , ty :: ConstKind :: Param (_) | ty :: ConstKind :: Infer (_) | ty :: ConstKind :: Bound (_ , _) | ty :: ConstKind :: Placeholder (_) | ty :: ConstKind :: Expr (_) => Err (EvaluateConstErr :: HasGenericsOrInfers) , ty :: ConstKind :: Unevaluated (uv) => { let opt_anon_const_kind = (tcx . def_kind (uv . def) == DefKind :: AnonConst) . then (| | tcx . anon_const_kind (uv . def)) ; let (args , typing_env) = match opt_anon_const_kind { Some (ty :: AnonConstKind :: GCE) => { if uv . has_non_region_infer () || uv . has_non_region_param () { match tcx . thir_abstract_const (uv . def) { Ok (Some (ct)) => { let ct = tcx . expand_abstract_consts (ct . instantiate (tcx , uv . args)) ; if let Err (e) = ct . error_reported () { return Err (EvaluateConstErr :: EvaluationFailure (e)) ; } else if ct . has_non_region_infer () || ct . has_non_region_param () { return Err (EvaluateConstErr :: HasGenericsOrInfers) ; } else { let args = replace_param_and_infer_args_with_placeholder (tcx , uv . args) ; let typing_env = infcx . typing_env (tcx . erase_and_anonymize_regions (param_env)) . with_post_analysis_normalized (tcx) ; (args , typing_env) } } Err (_) | Ok (None) => { let args = GenericArgs :: identity_for_item (tcx , uv . def) ; let typing_env = ty :: TypingEnv :: post_analysis (tcx , uv . def) ; (args , typing_env) } } } else { let typing_env = infcx . typing_env (tcx . erase_and_anonymize_regions (param_env)) . with_post_analysis_normalized (tcx) ; (uv . args , typing_env) } } Some (ty :: AnonConstKind :: RepeatExprCount) => { if uv . has_non_region_infer () { tcx . dcx () . delayed_bug ("AnonConst with infer args but no error reported") ; } let args = GenericArgs :: identity_for_item (tcx , uv . def) ; let typing_env = ty :: TypingEnv :: post_analysis (tcx , uv . def) ; (args , typing_env) } _ => { if uv . args . has_non_region_param () || uv . args . has_non_region_infer () { return Err (EvaluateConstErr :: HasGenericsOrInfers) ; } let typing_env = infcx . typing_env (tcx . erase_and_anonymize_regions (param_env)) . with_post_analysis_normalized (tcx) ; (uv . args , typing_env) } } ; let uv = ty :: UnevaluatedConst :: new (uv . def , args) ; let erased_uv = tcx . erase_and_anonymize_regions (uv) ; use rustc_middle :: mir :: interpret :: ErrorHandled ; match tcx . const_eval_resolve_for_typeck (typing_env , erased_uv , tcx . def_span (uv . def)) { Ok (Ok (val)) => Ok (ty :: Const :: new_value (tcx , val , tcx . type_of (uv . def) . instantiate (tcx , uv . args) ,)) , Ok (Err (_)) => { let e = tcx . dcx () . delayed_bug ("Type system constant with non valtree'able type evaluated but no error emitted" ,) ; Err (EvaluateConstErr :: InvalidConstParamTy (e)) } Err (ErrorHandled :: Reported (info , _)) => { Err (EvaluateConstErr :: EvaluationFailure (info . into ())) } Err (ErrorHandled :: TooGeneric (_)) => Err (EvaluateConstErr :: HasGenericsOrInfers) , } } } }
}

macro_rules! replace_param_and_infer_args_with_placeholder_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function replace_param_and_infer_args_with_placeholder in module {}", module_path!());
    };
}

mkfn!{
    replace_param_and_infer_args_with_placeholder_introspect!();
    # [doc = " Replaces args that reference param or infer variables with suitable"] # [doc = " placeholders. This function is meant to remove these param and infer"] # [doc = " args when they're not actually needed to evaluate a constant."] fn replace_param_and_infer_args_with_placeholder < 'tcx > (tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx > ,) -> GenericArgsRef < 'tcx > { struct ReplaceParamAndInferWithPlaceholder < 'tcx > { tcx : TyCtxt < 'tcx > , idx : ty :: BoundVar , } impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for ReplaceParamAndInferWithPlaceholder < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_ty (& mut self , t : Ty < 'tcx >) -> Ty < 'tcx > { if let ty :: Infer (_) = t . kind () { let idx = self . idx ; self . idx += 1 ; Ty :: new_placeholder (self . tcx , ty :: PlaceholderType { universe : ty :: UniverseIndex :: ROOT , bound : ty :: BoundTy { var : idx , kind : ty :: BoundTyKind :: Anon } , } ,) } else { t . super_fold_with (self) } } fn fold_const (& mut self , c : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { if let ty :: ConstKind :: Infer (_) = c . kind () { let idx = self . idx ; self . idx += 1 ; ty :: Const :: new_placeholder (self . tcx , ty :: PlaceholderConst { universe : ty :: UniverseIndex :: ROOT , bound : ty :: BoundConst { var : idx } , } ,) } else { c . super_fold_with (self) } } } args . fold_with (& mut ReplaceParamAndInferWithPlaceholder { tcx , idx : ty :: BoundVar :: ZERO }) }
}

macro_rules! impossible_predicates_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function impossible_predicates in module {}", module_path!());
    };
}

mkfn!{
    impossible_predicates_introspect!();
    # [doc = " Normalizes the predicates and checks whether they hold in an empty environment. If this"] # [doc = " returns true, then either normalize encountered an error or one of the predicates did not"] # [doc = " hold. Used when creating vtables to check for unsatisfiable methods. This should not be"] # [doc = " used during analysis."] pub fn impossible_predicates < 'tcx > (tcx : TyCtxt < 'tcx > , predicates : Vec < ty :: Clause < 'tcx > >) -> bool { debug ! ("impossible_predicates(predicates={:?})" , predicates) ; let (infcx , param_env) = tcx . infer_ctxt () . with_next_trait_solver (true) . build_with_typing_env (ty :: TypingEnv :: fully_monomorphized ()) ; let ocx = ObligationCtxt :: new (& infcx) ; let predicates = ocx . normalize (& ObligationCause :: dummy () , param_env , predicates) ; for predicate in predicates { let obligation = Obligation :: new (tcx , ObligationCause :: dummy () , param_env , predicate) ; ocx . register_obligation (obligation) ; } let true_errors = ocx . select_where_possible () ; if ! true_errors . is_empty () { return true ; } false }
}

macro_rules! instantiate_and_check_impossible_predicates_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function instantiate_and_check_impossible_predicates in module {}", module_path!());
    };
}

mkfn!{
    instantiate_and_check_impossible_predicates_introspect!();
    fn instantiate_and_check_impossible_predicates < 'tcx > (tcx : TyCtxt < 'tcx > , key : (DefId , GenericArgsRef < 'tcx >) ,) -> bool { debug ! ("instantiate_and_check_impossible_predicates(key={:?})" , key) ; let mut predicates = tcx . predicates_of (key . 0) . instantiate (tcx , key . 1) . predicates ; if let Some (trait_def_id) = tcx . trait_of_assoc (key . 0) { let trait_ref = ty :: TraitRef :: from_assoc (tcx , trait_def_id , key . 1) ; predicates . push (trait_ref . upcast (tcx)) ; } predicates . retain (| predicate | ! predicate . has_param ()) ; let result = impossible_predicates (tcx , predicates) ; debug ! ("instantiate_and_check_impossible_predicates(key={:?}) = {:?}" , key , result) ; result }
}

macro_rules! is_impossible_associated_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_impossible_associated_item in module {}", module_path!());
    };
}

mkfn!{
    is_impossible_associated_item_introspect!();
    # [doc = " Checks whether a trait's associated item is impossible to reference on a given impl."] # [doc = ""] # [doc = " This only considers predicates that reference the impl's generics, and not"] # [doc = " those that reference the method's generics."] fn is_impossible_associated_item (tcx : TyCtxt < '_ > , (impl_def_id , trait_item_def_id) : (DefId , DefId) ,) -> bool { struct ReferencesOnlyParentGenerics < 'tcx > { tcx : TyCtxt < 'tcx > , generics : & 'tcx ty :: Generics , trait_item_def_id : DefId , } impl < 'tcx > ty :: TypeVisitor < TyCtxt < 'tcx > > for ReferencesOnlyParentGenerics < 'tcx > { type Result = ControlFlow < () > ; fn visit_ty (& mut self , t : Ty < 'tcx >) -> Self :: Result { if let ty :: Param (param) = * t . kind () && let param_def_id = self . generics . type_param (param , self . tcx) . def_id && self . tcx . parent (param_def_id) == self . trait_item_def_id { return ControlFlow :: Break (()) ; } t . super_visit_with (self) } fn visit_region (& mut self , r : ty :: Region < 'tcx >) -> Self :: Result { if let ty :: ReEarlyParam (param) = r . kind () && let param_def_id = self . generics . region_param (param , self . tcx) . def_id && self . tcx . parent (param_def_id) == self . trait_item_def_id { return ControlFlow :: Break (()) ; } ControlFlow :: Continue (()) } fn visit_const (& mut self , ct : ty :: Const < 'tcx >) -> Self :: Result { if let ty :: ConstKind :: Param (param) = ct . kind () && let param_def_id = self . generics . const_param (param , self . tcx) . def_id && self . tcx . parent (param_def_id) == self . trait_item_def_id { return ControlFlow :: Break (()) ; } ct . super_visit_with (self) } } let generics = tcx . generics_of (trait_item_def_id) ; let predicates = tcx . predicates_of (trait_item_def_id) ; let infcx = tcx . infer_ctxt () . ignoring_regions () . with_next_trait_solver (true) . build (TypingMode :: Coherence) ; let param_env = ty :: ParamEnv :: empty () ; let fresh_args = infcx . fresh_args_for_item (tcx . def_span (impl_def_id) , impl_def_id) ; let impl_trait_ref = tcx . impl_trait_ref (impl_def_id) . expect ("expected impl to correspond to trait") . instantiate (tcx , fresh_args) ; let mut visitor = ReferencesOnlyParentGenerics { tcx , generics , trait_item_def_id } ; let predicates_for_trait = predicates . predicates . iter () . filter_map (| (pred , span) | { pred . visit_with (& mut visitor) . is_continue () . then (| | { Obligation :: new (tcx , ObligationCause :: dummy_with_span (* span) , param_env , ty :: EarlyBinder :: bind (* pred) . instantiate (tcx , impl_trait_ref . args) ,) }) }) ; let ocx = ObligationCtxt :: new (& infcx) ; ocx . register_obligations (predicates_for_trait) ; ! ocx . select_where_possible () . is_empty () }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (providers : & mut Providers) { dyn_compatibility :: provide (providers) ; vtable :: provide (providers) ; * providers = Providers { specialization_graph_of : specialize :: specialization_graph_provider , specializes : specialize :: specializes , specialization_enabled_in : specialize :: specialization_enabled_in , instantiate_and_check_impossible_predicates , is_impossible_associated_item , .. * providers } ; }
}