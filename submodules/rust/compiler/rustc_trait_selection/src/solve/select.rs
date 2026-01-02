mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_infer :: infer :: InferCtxt ;}
mkuse!{use rustc_infer :: traits :: solve :: inspect :: ProbeKind ;}
mkuse!{use rustc_infer :: traits :: solve :: { CandidateSource , Certainty , Goal } ;}
mkuse!{use rustc_infer :: traits :: { BuiltinImplSource , ImplSource , ImplSourceUserDefinedData , Obligation , ObligationCause , Selection , SelectionError , SelectionResult , TraitObligation , } ;}
mkuse!{use rustc_macros :: extension ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use thin_vec :: thin_vec ;}
mkuse!{use crate :: solve :: inspect :: { self , ProofTreeInferCtxtExt } ;}
mkitem!{mkimpl!{# [extension (pub trait InferCtxtSelectExt <'tcx >)] impl < 'tcx > InferCtxt < 'tcx > { # [doc = " Do not use this directly. This is called from [`crate::traits::SelectionContext::select`]."] fn select_in_new_trait_solver (& self , obligation : & TraitObligation < 'tcx > ,) -> SelectionResult < 'tcx , Selection < 'tcx > > { assert ! (self . next_trait_solver ()) ; self . visit_proof_tree (Goal :: new (self . tcx , obligation . param_env , obligation . predicate) , & mut Select { span : obligation . cause . span } ,) . break_value () . unwrap () } }}}
mkitem!{mkstruct!{struct Select { span : Span , }}}
mkitem!{mkimpl!{impl < 'tcx > inspect :: ProofTreeVisitor < 'tcx > for Select { type Result = ControlFlow < SelectionResult < 'tcx , Selection < 'tcx > > > ; fn span (& self) -> Span { self . span } fn visit_goal (& mut self , goal : & inspect :: InspectGoal < '_ , 'tcx >) -> Self :: Result { let mut candidates = goal . candidates () ; candidates . retain (| cand | cand . result () . is_ok ()) ; if candidates . is_empty () { return ControlFlow :: Break (Err (SelectionError :: Unimplemented)) ; } if candidates . len () == 1 { return ControlFlow :: Break (Ok (to_selection (self . span , candidates . into_iter () . next () . unwrap () ,))) ; } if matches ! (goal . result () . unwrap () , Certainty :: Maybe (..)) { return ControlFlow :: Break (Ok (None)) ; } let mut i = 0 ; while i < candidates . len () { let should_drop_i = (0 .. candidates . len ()) . filter (| & j | i != j) . any (| j | candidate_should_be_dropped_in_favor_of (& candidates [i] , & candidates [j])) ; if should_drop_i { candidates . swap_remove (i) ; } else { i += 1 ; if i > 1 { return ControlFlow :: Break (Ok (None)) ; } } } ControlFlow :: Break (Ok (to_selection (self . span , candidates . into_iter () . next () . unwrap ()))) } }}}

macro_rules! candidate_should_be_dropped_in_favor_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function candidate_should_be_dropped_in_favor_of in module {}", module_path!());
    };
}

mkfn!{
    candidate_should_be_dropped_in_favor_of_introspect!();
    # [doc = " This is a lot more limited than the old solver's equivalent method. This may lead to more `Ok(None)`"] # [doc = " results when selecting traits in polymorphic contexts, but we should never rely on the lack of ambiguity,"] # [doc = " and should always just gracefully fail here. We shouldn't rely on this incompleteness."] fn candidate_should_be_dropped_in_favor_of < 'tcx > (victim : & inspect :: InspectCandidate < '_ , 'tcx > , other : & inspect :: InspectCandidate < '_ , 'tcx > ,) -> bool { if matches ! (other . result () . unwrap () , Certainty :: Maybe (..)) { return false ; } let inspect :: ProbeKind :: TraitCandidate { source : victim_source , result : _ } = victim . kind () else { return false ; } ; let inspect :: ProbeKind :: TraitCandidate { source : other_source , result : _ } = other . kind () else { return false ; } ; match (victim_source , other_source) { (_ , CandidateSource :: CoherenceUnknowable) | (CandidateSource :: CoherenceUnknowable , _) => { bug ! ("should not have assembled a CoherenceUnknowable candidate") } (CandidateSource :: BuiltinImpl (BuiltinImplSource :: Object (a)) , CandidateSource :: BuiltinImpl (BuiltinImplSource :: Object (b)) ,) => a >= b , (CandidateSource :: BuiltinImpl (BuiltinImplSource :: TraitUpcasting (a)) , CandidateSource :: BuiltinImpl (BuiltinImplSource :: TraitUpcasting (b)) ,) => a >= b , (CandidateSource :: Impl (_) | CandidateSource :: ParamEnv (_) | CandidateSource :: AliasBound , CandidateSource :: BuiltinImpl (BuiltinImplSource :: Object { .. }) ,) => true , (CandidateSource :: Impl (victim_def_id) , CandidateSource :: Impl (other_def_id)) => { victim . goal () . infcx () . tcx . specializes ((other_def_id , victim_def_id)) } _ => false , } }
}

macro_rules! to_selection_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_selection in module {}", module_path!());
    };
}

mkfn!{
    to_selection_introspect!();
    fn to_selection < 'tcx > (span : Span , cand : inspect :: InspectCandidate < '_ , 'tcx > ,) -> Option < Selection < 'tcx > > { if let Certainty :: Maybe (..) = cand . shallow_certainty () { return None ; } let nested = match cand . result () . expect ("expected positive result") { Certainty :: Yes => thin_vec ! [] , Certainty :: Maybe (_) => cand . instantiate_nested_goals (span) . into_iter () . map (| nested | { Obligation :: new (nested . infcx () . tcx , ObligationCause :: dummy_with_span (span) , nested . goal () . param_env , nested . goal () . predicate ,) }) . collect () , } ; Some (match cand . kind () { ProbeKind :: TraitCandidate { source , result : _ } => match source { CandidateSource :: Impl (impl_def_id) => { ImplSource :: UserDefined (ImplSourceUserDefinedData { impl_def_id , args : cand . instantiate_impl_args (span) , nested , }) } CandidateSource :: BuiltinImpl (builtin) => ImplSource :: Builtin (builtin , nested) , CandidateSource :: ParamEnv (_) | CandidateSource :: AliasBound => ImplSource :: Param (nested) , CandidateSource :: CoherenceUnknowable => { span_bug ! (span , "didn't expect to select an unknowable candidate") } } , ProbeKind :: NormalizedSelfTyAssembly | ProbeKind :: UnsizeAssembly | ProbeKind :: ProjectionCompatibility | ProbeKind :: OpaqueTypeStorageLookup { result : _ } | ProbeKind :: Root { result : _ } | ProbeKind :: ShadowedEnvProbing | ProbeKind :: RigidAlias { result : _ } => { span_bug ! (span , "didn't expect to assemble trait candidate from {:#?}" , cand . kind ()) } }) }
}