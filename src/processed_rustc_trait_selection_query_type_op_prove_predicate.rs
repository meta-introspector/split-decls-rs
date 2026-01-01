/* FP:prove_predicate.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_prove_predicate_USE_0001
/* FP:prove_predicate.rs-0002 */ use crate :: rustc_infer :: traits :: Obligation ;
/* FP:prove_predicate.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_prove_predicate_USE_0002
/* FP:prove_predicate.rs-0004 */ use crate :: rustc_complete :: traits :: ObligationCause ;
/* FP:prove_predicate.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_prove_predicate_USE_0003
/* FP:prove_predicate.rs-0006 */ use crate :: rustc_complete :: traits :: query :: NoSolution ;
/* FP:prove_predicate.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_prove_predicate_USE_0004
/* FP:prove_predicate.rs-0008 */ pub use crate :: rustc_complete :: traits :: query :: type_op :: ProvePredicate ;
/* FP:prove_predicate.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_prove_predicate_USE_0005
/* FP:prove_predicate.rs-0010 */ use crate :: rustc_complete :: ty :: { self , ParamEnvAnd , TyCtxt } ;
/* FP:prove_predicate.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_prove_predicate_USE_0006
/* FP:prove_predicate.rs-0012 */ use crate :: rustc_complete :: Span ;
/* FP:prove_predicate.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_prove_predicate_USE_0007
/* FP:prove_predicate.rs-0014 */ use crate :: infer :: canonical :: { CanonicalQueryInput , CanonicalQueryResponse } ;
/* FP:prove_predicate.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_prove_predicate_USE_0008
/* FP:prove_predicate.rs-0016 */ use crate :: traits :: { ObligationCtxt , sizedness_fast_path } ;
/* FP:prove_predicate.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_prove_predicate_IMPL_0009
/* FP:prove_predicate.rs-0018 */ impl < 'tcx > super :: QueryTypeOp < 'tcx > for ProvePredicate < 'tcx > { type QueryResponse = () ; fn try_fast_path (tcx : TyCtxt < 'tcx > , key : & ParamEnvAnd < 'tcx , Self > ,) -> Option < Self :: QueryResponse > { if sizedness_fast_path (tcx , key . value . predicate , key . param_env) { return Some (()) ; } if let ty :: PredicateKind :: Clause (ty :: ClauseKind :: WellFormed (term)) = key . value . predicate . kind () . skip_binder () && term . is_trivially_wf (tcx) { return Some (()) ; } None } fn perform_query (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Self > > ,) -> Result < CanonicalQueryResponse < 'tcx , () > , NoSolution > { tcx . type_op_prove_predicate (canonicalized) } fn perform_locally_with_next_solver (ocx : & ObligationCtxt < '_ , 'tcx > , key : ParamEnvAnd < 'tcx , Self > , span : Span ,) -> Result < Self :: QueryResponse , NoSolution > { ocx . register_obligation (Obligation :: new (ocx . infcx . tcx , ObligationCause :: dummy_with_span (span) , key . param_env , key . value . predicate ,)) ; Ok (()) } }