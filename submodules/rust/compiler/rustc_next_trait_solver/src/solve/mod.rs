mkmod!{alias_relate, { 
                getname!(alias_relate);
                getsrc!(alias_relate);
                getpath!(alias_relate);
                get_deps!(alias_relate);
                get_crates!(alias_relate);
                mkinclude!(alias_relate);
                 
            }}
mkmod!{assembly, { 
                getname!(assembly);
                getsrc!(assembly);
                getpath!(assembly);
                get_deps!(assembly);
                get_crates!(assembly);
                mkinclude!(assembly);
                 
            }}
mkmod!{effect_goals, { 
                getname!(effect_goals);
                getsrc!(effect_goals);
                getpath!(effect_goals);
                get_deps!(effect_goals);
                get_crates!(effect_goals);
                mkinclude!(effect_goals);
                 
            }}
mkmod!{eval_ctxt, { 
                getname!(eval_ctxt);
                getsrc!(eval_ctxt);
                getpath!(eval_ctxt);
                get_deps!(eval_ctxt);
                get_crates!(eval_ctxt);
                mkinclude!(eval_ctxt);
                 
            }}
mkmod!{inspect, { 
                getname!(inspect);
                getsrc!(inspect);
                getpath!(inspect);
                get_deps!(inspect);
                get_crates!(inspect);
                mkinclude!(inspect);
                 
            }}
mkmod!{normalizes_to, { 
                getname!(normalizes_to);
                getsrc!(normalizes_to);
                getpath!(normalizes_to);
                get_deps!(normalizes_to);
                get_crates!(normalizes_to);
                mkinclude!(normalizes_to);
                 
            }}
mkmod!{project_goals, { 
                getname!(project_goals);
                getsrc!(project_goals);
                getpath!(project_goals);
                get_deps!(project_goals);
                get_crates!(project_goals);
                mkinclude!(project_goals);
                 
            }}
mkmod!{search_graph, { 
                getname!(search_graph);
                getsrc!(search_graph);
                getpath!(search_graph);
                get_deps!(search_graph);
                get_crates!(search_graph);
                mkinclude!(search_graph);
                 
            }}
mkmod!{trait_goals, { 
                getname!(trait_goals);
                getsrc!(trait_goals);
                getpath!(trait_goals);
                get_deps!(trait_goals);
                get_crates!(trait_goals);
                mkinclude!(trait_goals);
                 
            }}
mkuse!{use derive_where :: derive_where ;}
mkuse!{use rustc_type_ir :: inherent :: * ;}
mkuse!{pub use rustc_type_ir :: solve :: * ;}
mkuse!{use rustc_type_ir :: { self as ty , Interner , TyVid , TypingMode } ;}
mkuse!{use tracing :: instrument ;}
mkuse!{pub use self :: eval_ctxt :: { EvalCtxt , GenerateProofTree , SolverDelegateEvalExt , evaluate_root_goal_for_proof_tree_raw_provider , } ;}
mkuse!{use crate :: delegate :: SolverDelegate ;}
mkuse!{use crate :: solve :: assembly :: Candidate ;}
mkitem!{# [doc = " How many fixpoint iterations we should attempt inside of the solver before bailing"] # [doc = " with overflow."] # [doc = ""] # [doc = " We previously used  `cx.recursion_limit().0.checked_ilog2().unwrap_or(0)` for this."] # [doc = " However, it feels unlikely that uncreasing the recursion limit by a power of two"] # [doc = " to get one more itereation is every useful or desirable. We now instead used a constant"] # [doc = " here. If there ever ends up some use-cases where a bigger number of fixpoint iterations"] # [doc = " is required, we can add a new attribute for that or revert this to be dependant on the"] # [doc = " recursion limit again. However, this feels very unlikely."] const FIXPOINT_STEP_LIMIT : usize = 8 ;}
mkitem!{mkenum!{# [doc = " Whether evaluating this goal ended up changing the"] # [doc = " inference state."] # [derive (PartialEq , Eq , Debug , Hash , Clone , Copy)] pub enum HasChanged { Yes , No , }}}

macro_rules! has_no_inference_or_external_constraints_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_no_inference_or_external_constraints in module {}", module_path!());
    };
}

mkfn!{
    has_no_inference_or_external_constraints_introspect!();
    fn has_no_inference_or_external_constraints < I : Interner > (response : ty :: Canonical < I , Response < I > > ,) -> bool { let ExternalConstraintsData { ref region_constraints , ref opaque_types , ref normalization_nested_goals , } = * response . value . external_constraints ; response . value . var_values . is_identity () && region_constraints . is_empty () && opaque_types . is_empty () && normalization_nested_goals . is_empty () }
}

macro_rules! has_only_region_constraints_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_only_region_constraints in module {}", module_path!());
    };
}

mkfn!{
    has_only_region_constraints_introspect!();
    fn has_only_region_constraints < I : Interner > (response : ty :: Canonical < I , Response < I > >) -> bool { let ExternalConstraintsData { region_constraints : _ , ref opaque_types , ref normalization_nested_goals , } = * response . value . external_constraints ; response . value . var_values . is_identity_modulo_regions () && opaque_types . is_empty () && normalization_nested_goals . is_empty () }
}
mkitem!{mkimpl!{impl < 'a , D , I > EvalCtxt < 'a , D > where D : SolverDelegate < Interner = I > , I : Interner , { # [instrument (level = "trace" , skip (self))] fn compute_type_outlives_goal (& mut self , goal : Goal < I , ty :: OutlivesPredicate < I , I :: Ty > > ,) -> QueryResult < I > { let ty :: OutlivesPredicate (ty , lt) = goal . predicate ; self . register_ty_outlives (ty , lt) ; self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } # [instrument (level = "trace" , skip (self))] fn compute_region_outlives_goal (& mut self , goal : Goal < I , ty :: OutlivesPredicate < I , I :: Region > > ,) -> QueryResult < I > { let ty :: OutlivesPredicate (a , b) = goal . predicate ; self . register_region_outlives (a , b) ; self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } # [instrument (level = "trace" , skip (self))] fn compute_coerce_goal (& mut self , goal : Goal < I , ty :: CoercePredicate < I > >) -> QueryResult < I > { self . compute_subtype_goal (Goal { param_env : goal . param_env , predicate : ty :: SubtypePredicate { a_is_expected : false , a : goal . predicate . a , b : goal . predicate . b , } , }) } # [instrument (level = "trace" , skip (self))] fn compute_subtype_goal (& mut self , goal : Goal < I , ty :: SubtypePredicate < I > >) -> QueryResult < I > { match (goal . predicate . a . kind () , goal . predicate . b . kind ()) { (ty :: Infer (ty :: TyVar (a_vid)) , ty :: Infer (ty :: TyVar (b_vid))) => { self . sub_unify_ty_vids_raw (a_vid , b_vid) ; self . evaluate_added_goals_and_make_canonical_response (Certainty :: AMBIGUOUS) } _ => { self . sub (goal . param_env , goal . predicate . a , goal . predicate . b) ? ; self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } } } fn compute_dyn_compatible_goal (& mut self , trait_def_id : I :: TraitId) -> QueryResult < I > { if self . cx () . trait_is_dyn_compatible (trait_def_id) { self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } else { Err (NoSolution) } } # [instrument (level = "trace" , skip (self))] fn compute_well_formed_goal (& mut self , goal : Goal < I , I :: Term >) -> QueryResult < I > { match self . well_formed_goals (goal . param_env , goal . predicate) { Some (goals) => { self . add_goals (GoalSource :: Misc , goals) ; self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } None => self . evaluate_added_goals_and_make_canonical_response (Certainty :: AMBIGUOUS) , } } fn compute_unstable_feature_goal (& mut self , param_env : < I as Interner > :: ParamEnv , symbol : < I as Interner > :: Symbol ,) -> QueryResult < I > { if self . may_use_unstable_feature (param_env , symbol) { self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } else { self . evaluate_added_goals_and_make_canonical_response (Certainty :: Maybe (MaybeCause :: Ambiguity ,)) } } # [instrument (level = "trace" , skip (self))] fn compute_const_evaluatable_goal (& mut self , Goal { param_env , predicate : ct } : Goal < I , I :: Const > ,) -> QueryResult < I > { match ct . kind () { ty :: ConstKind :: Unevaluated (uv) => { if let Some (_normalized) = self . evaluate_const (param_env , uv) { self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } else { self . evaluate_added_goals_and_make_canonical_response (Certainty :: AMBIGUOUS) } } ty :: ConstKind :: Infer (_) => { self . evaluate_added_goals_and_make_canonical_response (Certainty :: AMBIGUOUS) } ty :: ConstKind :: Placeholder (_) | ty :: ConstKind :: Value (_) | ty :: ConstKind :: Error (_) => { self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } ty :: ConstKind :: Param (_) | ty :: ConstKind :: Bound (_ , _) | ty :: ConstKind :: Expr (_) => { panic ! ("unexpected const kind: {:?}" , ct) } } } # [instrument (level = "trace" , skip (self) , ret)] fn compute_const_arg_has_type_goal (& mut self , goal : Goal < I , (I :: Const , I :: Ty) > ,) -> QueryResult < I > { let (ct , ty) = goal . predicate ; let ct = self . structurally_normalize_const (goal . param_env , ct) ? ; let ct_ty = match ct . kind () { ty :: ConstKind :: Infer (_) => { return self . evaluate_added_goals_and_make_canonical_response (Certainty :: AMBIGUOUS) ; } ty :: ConstKind :: Error (_) => { return self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) ; } ty :: ConstKind :: Unevaluated (uv) => { self . cx () . type_of (uv . def) . instantiate (self . cx () , uv . args) } ty :: ConstKind :: Expr (_) => unimplemented ! ("`feature(generic_const_exprs)` is not supported in the new trait solver") , ty :: ConstKind :: Param (_) => { unreachable ! ("`ConstKind::Param` should have been canonicalized to `Placeholder`") } ty :: ConstKind :: Bound (_ , _) => panic ! ("escaping bound vars in {:?}" , ct) , ty :: ConstKind :: Value (cv) => cv . ty () , ty :: ConstKind :: Placeholder (placeholder) => { placeholder . find_const_ty_from_env (goal . param_env) } } ; self . eq (goal . param_env , ct_ty , ty) ? ; self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } }}}
mkitem!{mkenum!{# [derive (Debug)] enum MergeCandidateInfo { AlwaysApplicable (usize) , EqualResponse , }}}
mkitem!{mkimpl!{impl < D , I > EvalCtxt < '_ , D > where D : SolverDelegate < Interner = I > , I : Interner , { # [doc = " Try to merge multiple possible ways to prove a goal, if that is not possible returns `None`."] # [doc = ""] # [doc = " In this case we tend to flounder and return ambiguity by calling `[EvalCtxt::flounder]`."] # [instrument (level = "trace" , skip (self) , ret)] fn try_merge_candidates (& mut self , candidates : & [Candidate < I >] ,) -> Option < (CanonicalResponse < I > , MergeCandidateInfo) > { if candidates . is_empty () { return None ; } let always_applicable = candidates . iter () . enumerate () . find (| (_ , candidate) | { candidate . result . value . certainty == Certainty :: Yes && has_no_inference_or_external_constraints (candidate . result) }) ; if let Some ((i , c)) = always_applicable { return Some ((c . result , MergeCandidateInfo :: AlwaysApplicable (i))) ; } let one : CanonicalResponse < I > = candidates [0] . result ; if candidates [1 ..] . iter () . all (| candidate | candidate . result == one) { return Some ((one , MergeCandidateInfo :: EqualResponse)) ; } None } fn bail_with_ambiguity (& mut self , candidates : & [Candidate < I >]) -> CanonicalResponse < I > { debug_assert ! (candidates . len () > 1) ; let maybe_cause = candidates . iter () . fold (MaybeCause :: Ambiguity , | maybe_cause , candidates | { let candidate = match candidates . result . value . certainty { Certainty :: Yes => MaybeCause :: Ambiguity , Certainty :: Maybe (candidate) => candidate , } ; maybe_cause . or (candidate) }) ; self . make_ambiguous_response_no_constraints (maybe_cause) } # [doc = " If we fail to merge responses we flounder and return overflow or ambiguity."] # [instrument (level = "trace" , skip (self) , ret)] fn flounder (& mut self , candidates : & [Candidate < I >]) -> QueryResult < I > { if candidates . is_empty () { return Err (NoSolution) ; } else { Ok (self . bail_with_ambiguity (candidates)) } } # [doc = " Normalize a type for when it is structurally matched on."] # [doc = ""] # [doc = " This function is necessary in nearly all cases before matching on a type."] # [doc = " Not doing so is likely to be incomplete and therefore unsound during"] # [doc = " coherence."] # [instrument (level = "trace" , skip (self , param_env) , ret)] fn structurally_normalize_ty (& mut self , param_env : I :: ParamEnv , ty : I :: Ty ,) -> Result < I :: Ty , NoSolution > { self . structurally_normalize_term (param_env , ty . into ()) . map (| term | term . expect_ty ()) } # [doc = " Normalize a const for when it is structurally matched on, or more likely"] # [doc = " when it needs `.try_to_*` called on it (e.g. to turn it into a usize)."] # [doc = ""] # [doc = " This function is necessary in nearly all cases before matching on a const."] # [doc = " Not doing so is likely to be incomplete and therefore unsound during"] # [doc = " coherence."] # [instrument (level = "trace" , skip (self , param_env) , ret)] fn structurally_normalize_const (& mut self , param_env : I :: ParamEnv , ct : I :: Const ,) -> Result < I :: Const , NoSolution > { self . structurally_normalize_term (param_env , ct . into ()) . map (| term | term . expect_const ()) } # [doc = " Normalize a term for when it is structurally matched on."] # [doc = ""] # [doc = " This function is necessary in nearly all cases before matching on a ty/const."] # [doc = " Not doing so is likely to be incomplete and therefore unsound during coherence."] fn structurally_normalize_term (& mut self , param_env : I :: ParamEnv , term : I :: Term ,) -> Result < I :: Term , NoSolution > { if let Some (_) = term . to_alias_term () { let normalized_term = self . next_term_infer_of_kind (term) ; let alias_relate_goal = Goal :: new (self . cx () , param_env , ty :: PredicateKind :: AliasRelate (term , normalized_term , ty :: AliasRelationDirection :: Equate ,) ,) ; self . add_goal (GoalSource :: TypeRelating , alias_relate_goal) ; self . try_evaluate_added_goals () ? ; Ok (self . resolve_vars_if_possible (normalized_term)) } else { Ok (term) } } fn opaque_type_is_rigid (& self , def_id : I :: DefId) -> bool { match self . typing_mode () { TypingMode :: Coherence | TypingMode :: PostAnalysis => false , TypingMode :: Analysis { defining_opaque_types_and_generators : non_rigid_opaques } | TypingMode :: Borrowck { defining_opaque_types : non_rigid_opaques } | TypingMode :: PostBorrowckAnalysis { defined_opaque_types : non_rigid_opaques } => { ! def_id . as_local () . is_some_and (| def_id | non_rigid_opaques . contains (& def_id)) } } } }}}

macro_rules! response_no_constraints_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function response_no_constraints_raw in module {}", module_path!());
    };
}

mkfn!{
    response_no_constraints_raw_introspect!();
    fn response_no_constraints_raw < I : Interner > (cx : I , max_universe : ty :: UniverseIndex , variables : I :: CanonicalVarKinds , certainty : Certainty ,) -> CanonicalResponse < I > { ty :: Canonical { max_universe , variables , value : Response { var_values : ty :: CanonicalVarValues :: make_identity (cx , variables) , external_constraints : cx . mk_external_constraints (ExternalConstraintsData :: default ()) , certainty , } , } }
}
mkitem!{mkstruct!{# [doc = " The result of evaluating a goal."] pub struct GoalEvaluation < I : Interner > { # [doc = " The goal we've evaluated. This is the input goal, but potentially with its"] # [doc = " inference variables resolved. This never applies any inference constraints"] # [doc = " from evaluating the goal."] # [doc = ""] # [doc = " We rely on this to check whether root goals in HIR typeck had an unresolved"] # [doc = " type inference variable in the input. We must not resolve this after evaluating"] # [doc = " the goal as even if the inference variable has been resolved by evaluating the"] # [doc = " goal itself, this goal may still end up failing due to region uniquification"] # [doc = " later on."] # [doc = ""] # [doc = " This is used as a minor optimization to avoid re-resolving inference variables"] # [doc = " when reevaluating ambiguous goals. E.g. if we've got a goal `?x: Trait` with `?x`"] # [doc = " already being constrained to `Vec<?y>`, then the first evaluation resolves it to"] # [doc = " `Vec<?y>: Trait`. If this goal is still ambiguous and we later resolve `?y` to `u32`,"] # [doc = " then reevaluating this goal now only needs to resolve `?y` while it would otherwise"] # [doc = " have to resolve both `?x` and `?y`,"] pub goal : Goal < I , I :: Predicate > , pub certainty : Certainty , pub has_changed : HasChanged , # [doc = " If the [`Certainty`] was `Maybe`, then keep track of whether the goal has changed"] # [doc = " before rerunning it."] pub stalled_on : Option < GoalStalledOn < I > > , }}}
mkitem!{mkstruct!{# [doc = " The conditions that must change for a goal to warrant"] # [derive_where (Clone , Debug ; I : Interner)] pub struct GoalStalledOn < I : Interner > { pub num_opaques : usize , pub stalled_vars : Vec < I :: GenericArg > , pub sub_roots : Vec < TyVid > , # [doc = " The cause that will be returned on subsequent evaluations if this goal remains stalled."] pub stalled_cause : MaybeCause , }}}