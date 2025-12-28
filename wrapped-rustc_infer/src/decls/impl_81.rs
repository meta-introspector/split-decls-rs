macro_rules! deps {
    () => {
        Obligation!();
        InferCtxt!();
        OpaqueHiddenTypeDiag!();
        InferOk!();
        PredicateObligations!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < 'tcx > InferCtxt < 'tcx > { # [doc = " This is a backwards compatibility hack to prevent breaking changes from"] # [doc = " lazy TAIT around RPIT handling."] pub fn replace_opaque_types_with_inference_vars < T : TypeFoldable < TyCtxt < 'tcx > > > (& self , value : T , body_id : LocalDefId , span : Span , param_env : ty :: ParamEnv < 'tcx > ,) -> InferOk < 'tcx , T > { if self . next_trait_solver () { return InferOk { value , obligations : PredicateObligations :: new () } ; } if ! value . has_opaque_types () { return InferOk { value , obligations : PredicateObligations :: new () } ; } let mut obligations = PredicateObligations :: new () ; let value = value . fold_with (& mut BottomUpFolder { tcx : self . tcx , lt_op : | lt | lt , ct_op : | ct | ct , ty_op : | ty | match * ty . kind () { ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id , .. }) if self . can_define_opaque_ty (def_id) && ! ty . has_escaping_bound_vars () => { let def_span = self . tcx . def_span (def_id) ; let span = if span . contains (def_span) { def_span } else { span } ; let ty_var = self . next_ty_var (span) ; obligations . extend (self . handle_opaque_type (ty , ty_var , span , param_env) . unwrap () . into_iter () . map (| goal | { Obligation :: new (self . tcx , ObligationCause :: new (span , body_id , traits :: ObligationCauseCode :: OpaqueReturnType (None) ,) , goal . param_env , goal . predicate ,) }) ,) ; ty_var } _ => ty , } , }) ; InferOk { value , obligations } } pub fn handle_opaque_type (& self , a : Ty < 'tcx > , b : Ty < 'tcx > , span : Span , param_env : ty :: ParamEnv < 'tcx > ,) -> Result < Vec < Goal < 'tcx , ty :: Predicate < 'tcx > > > , TypeError < 'tcx > > { debug_assert ! (! self . next_trait_solver ()) ; let process = | a : Ty < 'tcx > , b : Ty < 'tcx > | match * a . kind () { ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id , args , .. }) if def_id . is_local () => { let def_id = def_id . expect_local () ; if let ty :: TypingMode :: Coherence = self . typing_mode () { return Some (self . register_hidden_type (OpaqueTypeKey { def_id , args } , span , param_env , b ,)) ; } if ! self . can_define_opaque_ty (def_id) { return None ; } if let ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id : b_def_id , .. }) = * b . kind () { if self . can_define_opaque_ty (b_def_id) && matches ! (self . tcx . opaque_ty_origin (b_def_id) , hir :: OpaqueTyOrigin :: TyAlias { .. }) { self . dcx () . emit_err (OpaqueHiddenTypeDiag { span , hidden_type : self . tcx . def_span (b_def_id) , opaque_type : self . tcx . def_span (def_id) , }) ; } } Some (self . register_hidden_type (OpaqueTypeKey { def_id , args } , span , param_env , b)) } _ => None , } ; if let Some (res) = process (a , b) { res } else if let Some (res) = process (b , a) { res } else { let (a , b) = self . resolve_vars_if_possible ((a , b)) ; Err (TypeError :: Sorts (ExpectedFound :: new (a , b))) } } }
    };
}

impl_81!();