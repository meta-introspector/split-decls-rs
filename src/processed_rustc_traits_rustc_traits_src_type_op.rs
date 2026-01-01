/* FP:type_op.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_USE_0001
/* FP:type_op.rs-0002 */ use std :: fmt ;
/* FP:type_op.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_USE_0002
/* FP:type_op.rs-0004 */ use crate :: rustc_infer :: infer :: TyCtxtInferExt ;
/* FP:type_op.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_USE_0003
/* FP:type_op.rs-0006 */ use crate :: rustc_infer :: infer :: canonical :: { Canonical , CanonicalQueryInput , QueryResponse } ;
/* FP:type_op.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_USE_0004
/* FP:type_op.rs-0008 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:type_op.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_USE_0005
/* FP:type_op.rs-0010 */ use crate :: rustc_complete :: traits :: query :: NoSolution ;
/* FP:type_op.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_USE_0006
/* FP:type_op.rs-0012 */ use crate :: rustc_complete :: ty :: { Clause , FnSig , ParamEnvAnd , PolyFnSig , Ty , TyCtxt , TypeFoldable } ;
/* FP:type_op.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_USE_0007
/* FP:type_op.rs-0014 */ use crate :: rustc_complete :: DUMMY_SP ;
/* FP:type_op.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_USE_0008
/* FP:type_op.rs-0016 */ use crate :: rustc_trait_selection :: infer :: InferCtxtBuilderExt ;
/* FP:type_op.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_USE_0009
/* FP:type_op.rs-0018 */ use crate :: rustc_trait_selection :: traits :: query :: normalize :: QueryNormalizeExt ;
/* FP:type_op.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_USE_0010
/* FP:type_op.rs-0020 */ use crate :: rustc_trait_selection :: traits :: query :: type_op :: ascribe_user_type :: { AscribeUserType , type_op_ascribe_user_type_with_span , } ;
/* FP:type_op.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_USE_0011
/* FP:type_op.rs-0022 */ use crate :: rustc_trait_selection :: traits :: query :: type_op :: normalize :: Normalize ;
/* FP:type_op.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_USE_0012
/* FP:type_op.rs-0024 */ use crate :: rustc_trait_selection :: traits :: query :: type_op :: prove_predicate :: ProvePredicate ;
/* FP:type_op.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_USE_0013
/* FP:type_op.rs-0026 */ use crate :: rustc_trait_selection :: traits :: { Normalized , Obligation , ObligationCause , ObligationCtxt } ;
/* FP:type_op.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_FN_0014
/* FP:type_op.rs-0028 */ pub (crate) fn provide (p : & mut Providers) { * p = Providers { type_op_ascribe_user_type , type_op_prove_predicate , type_op_normalize_ty , type_op_normalize_clause , type_op_normalize_fn_sig , type_op_normalize_poly_fn_sig , .. * p } ; }
/* FP:type_op.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_FN_0015
/* FP:type_op.rs-0030 */ fn type_op_ascribe_user_type < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , AscribeUserType < 'tcx > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , () > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , | ocx , key | { type_op_ascribe_user_type_with_span (ocx , key , DUMMY_SP) }) }
/* FP:type_op.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_FN_0016
/* FP:type_op.rs-0032 */ fn type_op_normalize < 'tcx , T > (ocx : & ObligationCtxt < '_ , 'tcx > , key : ParamEnvAnd < 'tcx , Normalize < T > > ,) -> Result < T , NoSolution > where T : fmt :: Debug + TypeFoldable < TyCtxt < 'tcx > > , { let ParamEnvAnd { param_env , value : Normalize { value } } = key ; let Normalized { value , obligations } = ocx . infcx . at (& ObligationCause :: dummy () , param_env) . query_normalize (value) ? ; ocx . register_obligations (obligations) ; Ok (value) }
/* FP:type_op.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_FN_0017
/* FP:type_op.rs-0034 */ fn type_op_normalize_ty < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Normalize < Ty < 'tcx > > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , Ty < 'tcx > > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , type_op_normalize) }
/* FP:type_op.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_FN_0018
/* FP:type_op.rs-0036 */ fn type_op_normalize_clause < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Normalize < Clause < 'tcx > > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , Clause < 'tcx > > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , type_op_normalize) }
/* FP:type_op.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_FN_0019
/* FP:type_op.rs-0038 */ fn type_op_normalize_fn_sig < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Normalize < FnSig < 'tcx > > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , FnSig < 'tcx > > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , type_op_normalize) }
/* FP:type_op.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_FN_0020
/* FP:type_op.rs-0040 */ fn type_op_normalize_poly_fn_sig < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Normalize < PolyFnSig < 'tcx > > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , PolyFnSig < 'tcx > > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , type_op_normalize) }
/* FP:type_op.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_FN_0021
/* FP:type_op.rs-0042 */ fn type_op_prove_predicate < 'tcx > (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , ProvePredicate < 'tcx > > > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , () > > , NoSolution > { tcx . infer_ctxt () . enter_canonical_trait_query (& canonicalized , | ocx , key | { type_op_prove_predicate_with_cause (ocx , key , ObligationCause :: dummy ()) ; Ok (()) }) }
/* FP:type_op.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_type_op_FN_0022
/* FP:type_op.rs-0044 */ # [doc = " The core of the `type_op_prove_predicate` query: for diagnostics purposes in NLL HRTB errors,"] # [doc = " this query can be re-run to better track the span of the obligation cause, and improve the error"] # [doc = " message. Do not call directly unless you're in that very specific context."] pub fn type_op_prove_predicate_with_cause < 'tcx > (ocx : & ObligationCtxt < '_ , 'tcx > , key : ParamEnvAnd < 'tcx , ProvePredicate < 'tcx > > , cause : ObligationCause < 'tcx > ,) { let ParamEnvAnd { param_env , value : ProvePredicate { predicate } } = key ; ocx . register_obligation (Obligation :: new (ocx . infcx . tcx , cause , param_env , predicate)) ; }