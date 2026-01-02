mkuse!{use std :: fmt :: Debug ;}
mkuse!{use rustc_data_structures :: stack :: ensure_sufficient_stack ;}
mkuse!{use rustc_infer :: infer :: InferCtxt ;}
mkuse!{use rustc_infer :: infer :: at :: At ;}
mkuse!{use rustc_infer :: traits :: solve :: Goal ;}
mkuse!{use rustc_infer :: traits :: { FromSolverError , Obligation , TraitEngine } ;}
mkuse!{use rustc_middle :: traits :: ObligationCause ;}
mkuse!{use rustc_middle :: ty :: { self , FallibleTypeFolder , Ty , TyCtxt , TypeFoldable , TypeFolder , TypeSuperFoldable , TypeVisitableExt , UniverseIndex , } ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use super :: { FulfillmentCtxt , NextSolverError } ;}
mkuse!{use crate :: error_reporting :: InferCtxtErrorExt ;}
mkuse!{use crate :: error_reporting :: traits :: OverflowCause ;}
mkuse!{use crate :: traits :: { BoundVarReplacer , PlaceholderReplacer , ScrubbedTraitError } ;}

macro_rules! deeply_normalize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function deeply_normalize in module {}", module_path!());
    };
}

mkfn!{
    deeply_normalize_introspect!();
    # [doc = " Deeply normalize all aliases in `value`. This does not handle inference and expects"] # [doc = " its input to be already fully resolved."] pub fn deeply_normalize < 'tcx , T , E > (at : At < '_ , 'tcx > , value : T) -> Result < T , Vec < E > > where T : TypeFoldable < TyCtxt < 'tcx > > , E : FromSolverError < 'tcx , NextSolverError < 'tcx > > , { assert ! (! value . has_escaping_bound_vars ()) ; deeply_normalize_with_skipped_universes (at , value , vec ! []) }
}

macro_rules! deeply_normalize_with_skipped_universes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function deeply_normalize_with_skipped_universes in module {}", module_path!());
    };
}

mkfn!{
    deeply_normalize_with_skipped_universes_introspect!();
    # [doc = " Deeply normalize all aliases in `value`. This does not handle inference and expects"] # [doc = " its input to be already fully resolved."] # [doc = ""] # [doc = " Additionally takes a list of universes which represents the binders which have been"] # [doc = " entered before passing `value` to the function. This is currently needed for"] # [doc = " `normalize_erasing_regions`, which skips binders as it walks through a type."] pub fn deeply_normalize_with_skipped_universes < 'tcx , T , E > (at : At < '_ , 'tcx > , value : T , universes : Vec < Option < UniverseIndex > > ,) -> Result < T , Vec < E > > where T : TypeFoldable < TyCtxt < 'tcx > > , E : FromSolverError < 'tcx , NextSolverError < 'tcx > > , { let (value , coroutine_goals) = deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals (at , value , universes ,) ? ; assert_eq ! (coroutine_goals , vec ! []) ; Ok (value) }
}

macro_rules! deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals in module {}", module_path!());
    };
}

mkfn!{
    deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals_introspect!();
    # [doc = " Deeply normalize all aliases in `value`. This does not handle inference and expects"] # [doc = " its input to be already fully resolved."] # [doc = ""] # [doc = " Additionally takes a list of universes which represents the binders which have been"] # [doc = " entered before passing `value` to the function. This is currently needed for"] # [doc = " `normalize_erasing_regions`, which skips binders as it walks through a type."] # [doc = ""] # [doc = " This returns a set of stalled obligations involving coroutines if the typing mode of"] # [doc = " the underlying infcx has any stalled coroutine def ids."] pub fn deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals < 'tcx , T , E > (at : At < '_ , 'tcx > , value : T , universes : Vec < Option < UniverseIndex > > ,) -> Result < (T , Vec < Goal < 'tcx , ty :: Predicate < 'tcx > > >) , Vec < E > > where T : TypeFoldable < TyCtxt < 'tcx > > , E : FromSolverError < 'tcx , NextSolverError < 'tcx > > , { let fulfill_cx = FulfillmentCtxt :: new (at . infcx) ; let mut folder = NormalizationFolder { at , fulfill_cx , depth : 0 , universes , stalled_coroutine_goals : vec ! [] , } ; let value = value . try_fold_with (& mut folder) ? ; let errors = folder . fulfill_cx . select_all_or_error (at . infcx) ; if errors . is_empty () { Ok ((value , folder . stalled_coroutine_goals)) } else { Err (errors) } }
}
mkitem!{mkstruct!{struct NormalizationFolder < 'me , 'tcx , E > { at : At < 'me , 'tcx > , fulfill_cx : FulfillmentCtxt < 'tcx , E > , depth : usize , universes : Vec < Option < UniverseIndex > > , stalled_coroutine_goals : Vec < Goal < 'tcx , ty :: Predicate < 'tcx > > > , }}}
mkitem!{mkimpl!{impl < 'tcx , E > NormalizationFolder < '_ , 'tcx , E > where E : FromSolverError < 'tcx , NextSolverError < 'tcx > > , { fn normalize_alias_term (& mut self , alias_term : ty :: Term < 'tcx > ,) -> Result < ty :: Term < 'tcx > , Vec < E > > { let infcx = self . at . infcx ; let tcx = infcx . tcx ; let recursion_limit = tcx . recursion_limit () ; if ! recursion_limit . value_within_limit (self . depth) { let term = alias_term . to_alias_term () . unwrap () ; self . at . infcx . err_ctxt () . report_overflow_error (OverflowCause :: DeeplyNormalize (term) , self . at . cause . span , true , | _ | { } ,) ; } self . depth += 1 ; let infer_term = infcx . next_term_var_of_kind (alias_term , self . at . cause . span) ; let obligation = Obligation :: new (tcx , self . at . cause . clone () , self . at . param_env , ty :: PredicateKind :: AliasRelate (alias_term . into () , infer_term . into () , ty :: AliasRelationDirection :: Equate ,) ,) ; self . fulfill_cx . register_predicate_obligation (infcx , obligation) ; self . select_all_and_stall_coroutine_predicates () ? ; let term = infcx . resolve_vars_if_possible (infer_term) ; let result = match term . kind () { ty :: TermKind :: Ty (ty) => ty . try_super_fold_with (self) ? . into () , ty :: TermKind :: Const (ct) => ct . try_super_fold_with (self) ? . into () , } ; self . depth -= 1 ; Ok (result) } fn select_all_and_stall_coroutine_predicates (& mut self) -> Result < () , Vec < E > > { let errors = self . fulfill_cx . select_where_possible (self . at . infcx) ; if ! errors . is_empty () { return Err (errors) ; } self . stalled_coroutine_goals . extend (self . fulfill_cx . drain_stalled_obligations_for_coroutines (self . at . infcx) . into_iter () . map (| obl | obl . as_goal ()) ,) ; let errors = self . fulfill_cx . collect_remaining_errors (self . at . infcx) ; if ! errors . is_empty () { return Err (errors) ; } Ok (()) } }}}
mkitem!{mkimpl!{impl < 'tcx , E > FallibleTypeFolder < TyCtxt < 'tcx > > for NormalizationFolder < '_ , 'tcx , E > where E : FromSolverError < 'tcx , NextSolverError < 'tcx > > + Debug , { type Error = Vec < E > ; fn cx (& self) -> TyCtxt < 'tcx > { self . at . infcx . tcx } fn try_fold_binder < T : TypeFoldable < TyCtxt < 'tcx > > > (& mut self , t : ty :: Binder < 'tcx , T > ,) -> Result < ty :: Binder < 'tcx , T > , Self :: Error > { self . universes . push (None) ; let t = t . try_super_fold_with (self) ? ; self . universes . pop () ; Ok (t) } # [instrument (level = "trace" , skip (self) , ret)] fn try_fold_ty (& mut self , ty : Ty < 'tcx >) -> Result < Ty < 'tcx > , Self :: Error > { let infcx = self . at . infcx ; debug_assert_eq ! (ty , infcx . shallow_resolve (ty)) ; if ! ty . has_aliases () { return Ok (ty) ; } let ty :: Alias (..) = * ty . kind () else { return ty . try_super_fold_with (self) } ; if ty . has_escaping_bound_vars () { let (ty , mapped_regions , mapped_types , mapped_consts) = BoundVarReplacer :: replace_bound_vars (infcx , & mut self . universes , ty) ; let result = ensure_sufficient_stack (| | self . normalize_alias_term (ty . into ())) ? . expect_type () ; Ok (PlaceholderReplacer :: replace_placeholders (infcx , mapped_regions , mapped_types , mapped_consts , & self . universes , result ,)) } else { Ok (ensure_sufficient_stack (| | self . normalize_alias_term (ty . into ())) ? . expect_type ()) } } # [instrument (level = "trace" , skip (self) , ret)] fn try_fold_const (& mut self , ct : ty :: Const < 'tcx >) -> Result < ty :: Const < 'tcx > , Self :: Error > { let infcx = self . at . infcx ; debug_assert_eq ! (ct , infcx . shallow_resolve_const (ct)) ; if ! ct . has_aliases () { return Ok (ct) ; } let ty :: ConstKind :: Unevaluated (..) = ct . kind () else { return ct . try_super_fold_with (self) } ; if ct . has_escaping_bound_vars () { let (ct , mapped_regions , mapped_types , mapped_consts) = BoundVarReplacer :: replace_bound_vars (infcx , & mut self . universes , ct) ; let result = ensure_sufficient_stack (| | self . normalize_alias_term (ct . into ())) ? . expect_const () ; Ok (PlaceholderReplacer :: replace_placeholders (infcx , mapped_regions , mapped_types , mapped_consts , & self . universes , result ,)) } else { Ok (ensure_sufficient_stack (| | self . normalize_alias_term (ct . into ())) ? . expect_const ()) } } }}}

macro_rules! deeply_normalize_for_diagnostics_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function deeply_normalize_for_diagnostics in module {}", module_path!());
    };
}

mkfn!{
    deeply_normalize_for_diagnostics_introspect!();
    pub (crate) fn deeply_normalize_for_diagnostics < 'tcx , T : TypeFoldable < TyCtxt < 'tcx > > > (infcx : & InferCtxt < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , t : T ,) -> T { t . fold_with (& mut DeeplyNormalizeForDiagnosticsFolder { at : infcx . at (& ObligationCause :: dummy () , param_env) , }) }
}
mkitem!{mkstruct!{struct DeeplyNormalizeForDiagnosticsFolder < 'a , 'tcx > { at : At < 'a , 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for DeeplyNormalizeForDiagnosticsFolder < '_ , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . at . infcx . tcx } fn fold_ty (& mut self , ty : Ty < 'tcx >) -> Ty < 'tcx > { let infcx = self . at . infcx ; let result : Result < _ , Vec < ScrubbedTraitError < 'tcx > > > = infcx . commit_if_ok (| _ | { deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals (self . at , ty , vec ! [None ; ty . outer_exclusive_binder () . as_usize ()] ,) }) ; match result { Ok ((ty , _)) => ty , Err (_) => ty . super_fold_with (self) , } } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { let infcx = self . at . infcx ; let result : Result < _ , Vec < ScrubbedTraitError < 'tcx > > > = infcx . commit_if_ok (| _ | { deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals (self . at , ct , vec ! [None ; ct . outer_exclusive_binder () . as_usize ()] ,) }) ; match result { Ok ((ct , _)) => ct , Err (_) => ct . super_fold_with (self) , } } }}}