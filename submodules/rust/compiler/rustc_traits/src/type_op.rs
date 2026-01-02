mkuse!{use std :: fmt ;}
mkuse!{use rustc_infer :: infer :: TyCtxtInferExt ;}
mkuse!{use rustc_infer :: infer :: canonical :: { Canonical , CanonicalQueryInput , QueryResponse } ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: traits :: query :: NoSolution ;}
mkuse!{use rustc_middle :: ty :: { Clause , FnSig , ParamEnvAnd , PolyFnSig , Ty , TyCtxt , TypeFoldable } ;}
mkuse!{use rustc_span :: DUMMY_SP ;}
mkuse!{use rustc_trait_selection :: infer :: InferCtxtBuilderExt ;}
mkuse!{use rustc_trait_selection :: traits :: query :: normalize :: QueryNormalizeExt ;}
mkuse!{use rustc_trait_selection :: traits :: query :: type_op :: ascribe_user_type :: { AscribeUserType , type_op_ascribe_user_type_with_span , } ;}
mkuse!{use rustc_trait_selection :: traits :: query :: type_op :: normalize :: Normalize ;}
mkuse!{use rustc_trait_selection :: traits :: query :: type_op :: prove_predicate :: ProvePredicate ;}
mkuse!{use rustc_trait_selection :: traits :: { Normalized , Obligation , ObligationCause , ObligationCtxt } ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (p : & mut Providers) { * p = Providers { type_op_ascribe_user_type , type_op_prove_predicate , type_op_normalize_ty , type_op_normalize_clause , type_op_normalize_fn_sig , type_op_normalize_poly_fn_sig , .. * p } ; }
}

macro_rules! type_op_ascribe_user_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_op_ascribe_user_type in module {}", module_path!());
    };
}

mkfn!{
    type_op_ascribe_user_type_introspect!();
    fn type_op_ascribe_user_type < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , AscribeUserType < 'tcx > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , () > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , | ocx , key | { type_op_ascribe_user_type_with_span (ocx , key , DUMMY_SP) }) }
}

macro_rules! type_op_normalize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_op_normalize in module {}", module_path!());
    };
}

mkfn!{
    type_op_normalize_introspect!();
    fn type_op_normalize < 'tcx , T > (ocx : & ObligationCtxt < '_ , 'tcx > , key : ParamEnvAnd < 'tcx , Normalize < T > > ,) -> Result < T , NoSolution > where T : fmt :: Debug + TypeFoldable < TyCtxt < 'tcx > > , { let ParamEnvAnd { param_env , value : Normalize { value } } = key ; let Normalized { value , obligations } = ocx . infcx . at (& ObligationCause :: dummy () , param_env) . query_normalize (value) ? ; ocx . register_obligations (obligations) ; Ok (value) }
}

macro_rules! type_op_normalize_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_op_normalize_ty in module {}", module_path!());
    };
}

mkfn!{
    type_op_normalize_ty_introspect!();
    fn type_op_normalize_ty < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Normalize < Ty < 'tcx > > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , Ty < 'tcx > > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , type_op_normalize) }
}

macro_rules! type_op_normalize_clause_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_op_normalize_clause in module {}", module_path!());
    };
}

mkfn!{
    type_op_normalize_clause_introspect!();
    fn type_op_normalize_clause < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Normalize < Clause < 'tcx > > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , Clause < 'tcx > > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , type_op_normalize) }
}

macro_rules! type_op_normalize_fn_sig_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_op_normalize_fn_sig in module {}", module_path!());
    };
}

mkfn!{
    type_op_normalize_fn_sig_introspect!();
    fn type_op_normalize_fn_sig < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Normalize < FnSig < 'tcx > > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , FnSig < 'tcx > > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , type_op_normalize) }
}

macro_rules! type_op_normalize_poly_fn_sig_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_op_normalize_poly_fn_sig in module {}", module_path!());
    };
}

mkfn!{
    type_op_normalize_poly_fn_sig_introspect!();
    fn type_op_normalize_poly_fn_sig < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Normalize < PolyFnSig < 'tcx > > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , PolyFnSig < 'tcx > > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , type_op_normalize) }
}

macro_rules! type_op_prove_predicate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_op_prove_predicate in module {}", module_path!());
    };
}

mkfn!{
    type_op_prove_predicate_introspect!();
    fn type_op_prove_predicate < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , ProvePredicate < 'tcx > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , () > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , | ocx , key | { type_op_prove_predicate_with_cause (ocx , key , ObligationCause :: dummy ()) ; Ok (()) }) }
}

macro_rules! type_op_prove_predicate_with_cause_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_op_prove_predicate_with_cause in module {}", module_path!());
    };
}

mkfn!{
    type_op_prove_predicate_with_cause_introspect!();
    # [doc = " The core of the `type_op_prove_predicate` query: for diagnostics purposes in NLL HRTB errors,"] # [doc = " this query can be re-run to better track the span of the obligation cause, and improve the error"] # [doc = " message. Do not call directly unless you're in that very specific context."] pub fn type_op_prove_predicate_with_cause < 'tcx > (ocx : & ObligationCtxt < '_ , 'tcx > , key : ParamEnvAnd < 'tcx , ProvePredicate < 'tcx > > , cause : ObligationCause < 'tcx > ,) { let ParamEnvAnd { param_env , value : ProvePredicate { predicate } } = key ; ocx . register_obligation (Obligation :: new (ocx . infcx . tcx , cause , param_env , predicate)) ; }
}