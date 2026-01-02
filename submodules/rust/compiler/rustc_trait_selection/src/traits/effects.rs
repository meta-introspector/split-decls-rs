mkuse!{use rustc_hir :: { self as hir , LangItem } ;}
mkuse!{use rustc_infer :: infer :: { BoundRegionConversionTime , DefineOpaqueTypes } ;}
mkuse!{use rustc_infer :: traits :: { ImplDerivedHostCause , ImplSource , Obligation , ObligationCause , ObligationCauseCode , PredicateObligation , } ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: traits :: query :: NoSolution ;}
mkuse!{use rustc_middle :: ty :: elaborate :: elaborate ;}
mkuse!{use rustc_middle :: ty :: fast_reject :: DeepRejectCtxt ;}
mkuse!{use rustc_middle :: ty :: { self , TypingMode } ;}
mkuse!{use thin_vec :: { ThinVec , thin_vec } ;}
mkuse!{use super :: SelectionContext ;}
mkuse!{use super :: normalize :: normalize_with_depth_to ;}
mkitem!{pub type HostEffectObligation < 'tcx > = Obligation < 'tcx , ty :: HostEffectPredicate < 'tcx > > ;}
mkitem!{mkenum!{pub enum EvaluationFailure { Ambiguous , NoSolution , }}}

macro_rules! evaluate_host_effect_obligation_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_obligation in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_obligation_introspect!();
    pub fn evaluate_host_effect_obligation < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { if matches ! (selcx . infcx . typing_mode () , TypingMode :: Coherence) { span_bug ! (obligation . cause . span , "should not select host obligation in old solver in intercrate mode") ; } let ref obligation = selcx . infcx . resolve_vars_if_possible (obligation . clone ()) ; if obligation . predicate . self_ty () . is_ty_var () { return Err (EvaluationFailure :: Ambiguous) ; } match evaluate_host_effect_from_bounds (selcx , obligation) { Ok (result) => return Ok (result) , Err (EvaluationFailure :: Ambiguous) => return Err (EvaluationFailure :: Ambiguous) , Err (EvaluationFailure :: NoSolution) => { } } match evaluate_host_effect_from_conditionally_const_item_bounds (selcx , obligation) { Ok (result) => return Ok (result) , Err (EvaluationFailure :: Ambiguous) => return Err (EvaluationFailure :: Ambiguous) , Err (EvaluationFailure :: NoSolution) => { } } match evaluate_host_effect_from_item_bounds (selcx , obligation) { Ok (result) => return Ok (result) , Err (EvaluationFailure :: Ambiguous) => return Err (EvaluationFailure :: Ambiguous) , Err (EvaluationFailure :: NoSolution) => { } } match evaluate_host_effect_from_builtin_impls (selcx , obligation) { Ok (result) => return Ok (result) , Err (EvaluationFailure :: Ambiguous) => return Err (EvaluationFailure :: Ambiguous) , Err (EvaluationFailure :: NoSolution) => { } } match evaluate_host_effect_from_selection_candidate (selcx , obligation) { Ok (result) => return Ok (result) , Err (EvaluationFailure :: Ambiguous) => return Err (EvaluationFailure :: Ambiguous) , Err (EvaluationFailure :: NoSolution) => { } } Err (EvaluationFailure :: NoSolution) }
}

macro_rules! match_candidate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function match_candidate in module {}", module_path!());
    };
}

mkfn!{
    match_candidate_introspect!();
    fn match_candidate < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > , candidate : ty :: Binder < 'tcx , ty :: HostEffectPredicate < 'tcx > > , candidate_is_unnormalized : bool , more_nested : impl FnOnce (& mut SelectionContext < '_ , 'tcx > , & mut ThinVec < PredicateObligation < 'tcx > >) ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , NoSolution > { if ! candidate . skip_binder () . constness . satisfies (obligation . predicate . constness) { return Err (NoSolution) ; } let mut candidate = selcx . infcx . instantiate_binder_with_fresh_vars (obligation . cause . span , BoundRegionConversionTime :: HigherRankedType , candidate ,) ; let mut nested = thin_vec ! [] ; if candidate_is_unnormalized { candidate = normalize_with_depth_to (selcx , obligation . param_env , obligation . cause . clone () , obligation . recursion_depth , candidate , & mut nested ,) ; } nested . extend (selcx . infcx . at (& obligation . cause , obligation . param_env) . eq (DefineOpaqueTypes :: Yes , obligation . predicate . trait_ref , candidate . trait_ref) ? . into_obligations () ,) ; more_nested (selcx , & mut nested) ; Ok (nested) }
}

macro_rules! evaluate_host_effect_from_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_from_bounds in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_from_bounds_introspect!();
    fn evaluate_host_effect_from_bounds < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { let infcx = selcx . infcx ; let drcx = DeepRejectCtxt :: relate_rigid_rigid (selcx . tcx ()) ; let mut candidate = None ; for clause in obligation . param_env . caller_bounds () { let bound_clause = clause . kind () ; let ty :: ClauseKind :: HostEffect (data) = bound_clause . skip_binder () else { continue ; } ; let data = bound_clause . rebind (data) ; if data . skip_binder () . trait_ref . def_id != obligation . predicate . trait_ref . def_id { continue ; } if ! drcx . args_may_unify (obligation . predicate . trait_ref . args , data . skip_binder () . trait_ref . args) { continue ; } let is_match = infcx . probe (| _ | match_candidate (selcx , obligation , data , false , | _ , _ | { }) . is_ok ()) ; if is_match { if candidate . is_some () { return Err (EvaluationFailure :: Ambiguous) ; } else { candidate = Some (data) ; } } } if let Some (data) = candidate { Ok (match_candidate (selcx , obligation , data , false , | _ , _ | { }) . expect ("candidate matched before, so it should match again")) } else { Err (EvaluationFailure :: NoSolution) } }
}

macro_rules! evaluate_host_effect_from_conditionally_const_item_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_from_conditionally_const_item_bounds in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_from_conditionally_const_item_bounds_introspect!();
    # [doc = " Assembles constness bounds from `~const` item bounds on alias types, which only"] # [doc = " hold if the `~const` where bounds also hold and the parent trait is `~const`."] fn evaluate_host_effect_from_conditionally_const_item_bounds < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { let infcx = selcx . infcx ; let tcx = infcx . tcx ; let drcx = DeepRejectCtxt :: relate_rigid_rigid (selcx . tcx ()) ; let mut candidate = None ; let mut consider_ty = obligation . predicate . self_ty () ; while let ty :: Alias (kind @ (ty :: Projection | ty :: Opaque) , alias_ty) = * consider_ty . kind () { if tcx . is_conditionally_const (alias_ty . def_id) { for clause in elaborate (tcx , tcx . explicit_implied_const_bounds (alias_ty . def_id) . iter_instantiated_copied (tcx , alias_ty . args) . map (| (trait_ref , _) | { trait_ref . to_host_effect_clause (tcx , obligation . predicate . constness) }) ,) { let bound_clause = clause . kind () ; let ty :: ClauseKind :: HostEffect (data) = bound_clause . skip_binder () else { unreachable ! ("should not elaborate non-HostEffect from HostEffect") } ; let data = bound_clause . rebind (data) ; if data . skip_binder () . trait_ref . def_id != obligation . predicate . trait_ref . def_id { continue ; } if ! drcx . args_may_unify (obligation . predicate . trait_ref . args , data . skip_binder () . trait_ref . args ,) { continue ; } let is_match = infcx . probe (| _ | match_candidate (selcx , obligation , data , true , | _ , _ | { }) . is_ok ()) ; if is_match { if candidate . is_some () { return Err (EvaluationFailure :: Ambiguous) ; } else { candidate = Some ((data , alias_ty)) ; } } } } if kind != ty :: Projection { break ; } consider_ty = alias_ty . self_ty () ; } if let Some ((data , alias_ty)) = candidate { Ok (match_candidate (selcx , obligation , data , true , | selcx , nested | { let const_conditions = normalize_with_depth_to (selcx , obligation . param_env , obligation . cause . clone () , obligation . recursion_depth , tcx . const_conditions (alias_ty . def_id) . instantiate (tcx , alias_ty . args) , nested ,) ; nested . extend (const_conditions . into_iter () . map (| (trait_ref , _) | { obligation . with (tcx , trait_ref . to_host_effect_clause (tcx , obligation . predicate . constness)) })) ; }) . expect ("candidate matched before, so it should match again")) } else { Err (EvaluationFailure :: NoSolution) } }
}

macro_rules! evaluate_host_effect_from_item_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_from_item_bounds in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_from_item_bounds_introspect!();
    # [doc = " Assembles constness bounds \"normal\" item bounds on aliases, which may include"] # [doc = " unconditionally `const` bounds that are *not* conditional and thus always hold."] fn evaluate_host_effect_from_item_bounds < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { let infcx = selcx . infcx ; let tcx = infcx . tcx ; let drcx = DeepRejectCtxt :: relate_rigid_rigid (selcx . tcx ()) ; let mut candidate = None ; let mut consider_ty = obligation . predicate . self_ty () ; while let ty :: Alias (kind @ (ty :: Projection | ty :: Opaque) , alias_ty) = * consider_ty . kind () { for clause in tcx . item_bounds (alias_ty . def_id) . iter_instantiated (tcx , alias_ty . args) { let bound_clause = clause . kind () ; let ty :: ClauseKind :: HostEffect (data) = bound_clause . skip_binder () else { continue ; } ; let data = bound_clause . rebind (data) ; if data . skip_binder () . trait_ref . def_id != obligation . predicate . trait_ref . def_id { continue ; } if ! drcx . args_may_unify (obligation . predicate . trait_ref . args , data . skip_binder () . trait_ref . args ,) { continue ; } let is_match = infcx . probe (| _ | match_candidate (selcx , obligation , data , true , | _ , _ | { }) . is_ok ()) ; if is_match { if candidate . is_some () { return Err (EvaluationFailure :: Ambiguous) ; } else { candidate = Some (data) ; } } } if kind != ty :: Projection { break ; } consider_ty = alias_ty . self_ty () ; } if let Some (data) = candidate { Ok (match_candidate (selcx , obligation , data , true , | _ , _ | { }) . expect ("candidate matched before, so it should match again")) } else { Err (EvaluationFailure :: NoSolution) } }
}

macro_rules! evaluate_host_effect_from_builtin_impls_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_from_builtin_impls in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_from_builtin_impls_introspect!();
    fn evaluate_host_effect_from_builtin_impls < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { match selcx . tcx () . as_lang_item (obligation . predicate . def_id ()) { Some (LangItem :: Destruct) => evaluate_host_effect_for_destruct_goal (selcx , obligation) , Some (LangItem :: Fn | LangItem :: FnMut | LangItem :: FnOnce) => { evaluate_host_effect_for_fn_goal (selcx , obligation) } _ => Err (EvaluationFailure :: NoSolution) , } }
}

macro_rules! evaluate_host_effect_for_destruct_goal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_for_destruct_goal in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_for_destruct_goal_introspect!();
    fn evaluate_host_effect_for_destruct_goal < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { let tcx = selcx . tcx () ; let destruct_def_id = tcx . require_lang_item (LangItem :: Destruct , obligation . cause . span) ; let self_ty = obligation . predicate . self_ty () ; let const_conditions = match * self_ty . kind () { ty :: Adt (adt_def , _) if adt_def . is_manually_drop () => thin_vec ! [] , ty :: Adt (adt_def , args) => { let mut const_conditions : ThinVec < _ > = adt_def . all_fields () . map (| field | ty :: TraitRef :: new (tcx , destruct_def_id , [field . ty (tcx , args)])) . collect () ; match adt_def . destructor (tcx) . map (| dtor | tcx . constness (dtor . did)) { Some (hir :: Constness :: NotConst) => return Err (EvaluationFailure :: NoSolution) , Some (hir :: Constness :: Const) => { let drop_def_id = tcx . require_lang_item (LangItem :: Drop , obligation . cause . span) ; let drop_trait_ref = ty :: TraitRef :: new (tcx , drop_def_id , [self_ty]) ; const_conditions . push (drop_trait_ref) ; } None => { } } const_conditions } ty :: Array (ty , _) | ty :: Pat (ty , _) | ty :: Slice (ty) => { thin_vec ! [ty :: TraitRef :: new (tcx , destruct_def_id , [ty])] } ty :: Tuple (tys) => { tys . iter () . map (| field_ty | ty :: TraitRef :: new (tcx , destruct_def_id , [field_ty])) . collect () } ty :: Bool | ty :: Char | ty :: Int (..) | ty :: Uint (..) | ty :: Float (..) | ty :: Str | ty :: RawPtr (..) | ty :: Ref (..) | ty :: FnDef (..) | ty :: FnPtr (..) | ty :: Never | ty :: Infer (ty :: InferTy :: FloatVar (_) | ty :: InferTy :: IntVar (_)) | ty :: Error (_) => thin_vec ! [] , ty :: Closure (_ , _) | ty :: CoroutineClosure (_ , _) | ty :: Coroutine (_ , _) | ty :: CoroutineWitness (_ , _) => return Err (EvaluationFailure :: NoSolution) , ty :: UnsafeBinder (_) => return Err (EvaluationFailure :: NoSolution) , ty :: Dynamic (..) | ty :: Param (_) | ty :: Alias (..) | ty :: Placeholder (_) | ty :: Foreign (_) => { return Err (EvaluationFailure :: NoSolution) ; } ty :: Bound (..) | ty :: Infer (ty :: TyVar (_) | ty :: FreshTy (_) | ty :: FreshIntTy (_) | ty :: FreshFloatTy (_)) => { panic ! ("unexpected type `{self_ty:?}`") } } ; Ok (const_conditions . into_iter () . map (| trait_ref | { obligation . with (tcx , ty :: Binder :: dummy (trait_ref) . to_host_effect_clause (tcx , obligation . predicate . constness) ,) }) . collect ()) }
}

macro_rules! evaluate_host_effect_for_fn_goal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_for_fn_goal in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_for_fn_goal_introspect!();
    fn evaluate_host_effect_for_fn_goal < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { let tcx = selcx . tcx () ; let self_ty = obligation . predicate . self_ty () ; let (def , args) = match * self_ty . kind () { ty :: FnDef (def , args) => (def , args) , ty :: FnPtr (..) => return Err (EvaluationFailure :: NoSolution) , ty :: Closure (..) | ty :: CoroutineClosure (_ , _) => { return Err (EvaluationFailure :: NoSolution) ; } _ => return Err (EvaluationFailure :: NoSolution) , } ; match tcx . constness (def) { hir :: Constness :: Const => Ok (tcx . const_conditions (def) . instantiate (tcx , args) . into_iter () . map (| (c , span) | { let code = ObligationCauseCode :: WhereClause (def , span) ; let cause = ObligationCause :: new (obligation . cause . span , obligation . cause . body_id , code) ; Obligation :: new (tcx , cause , obligation . param_env , c . to_host_effect_clause (tcx , obligation . predicate . constness) ,) }) . collect ()) , hir :: Constness :: NotConst => Err (EvaluationFailure :: NoSolution) , } }
}

macro_rules! evaluate_host_effect_from_selection_candidate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_from_selection_candidate in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_from_selection_candidate_introspect!();
    fn evaluate_host_effect_from_selection_candidate < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { let tcx = selcx . tcx () ; selcx . infcx . commit_if_ok (| _ | { match selcx . select (& obligation . with (tcx , obligation . predicate . trait_ref)) { Ok (None) => Err (EvaluationFailure :: Ambiguous) , Err (_) => Err (EvaluationFailure :: NoSolution) , Ok (Some (source)) => match source { ImplSource :: UserDefined (impl_) => { if tcx . impl_trait_header (impl_ . impl_def_id) . unwrap () . constness != hir :: Constness :: Const { return Err (EvaluationFailure :: NoSolution) ; } let mut nested = impl_ . nested ; nested . extend (tcx . const_conditions (impl_ . impl_def_id) . instantiate (tcx , impl_ . args) . into_iter () . map (| (trait_ref , span) | { Obligation :: new (tcx , obligation . cause . clone () . derived_host_cause (ty :: Binder :: dummy (obligation . predicate) , | derived | { ObligationCauseCode :: ImplDerivedHost (Box :: new (ImplDerivedHostCause { derived , impl_def_id : impl_ . impl_def_id , span , } ,)) } ,) , obligation . param_env , trait_ref . to_host_effect_clause (tcx , obligation . predicate . constness) ,) }) ,) ; Ok (nested) } _ => Err (EvaluationFailure :: NoSolution) , } , } }) }
}