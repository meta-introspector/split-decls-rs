/* FP:outlives.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_outlives_USE_0001
/* FP:outlives.rs-0002 */ use crate :: rustc_complete :: traits :: query :: { DropckOutlivesResult , NoSolution } ;
/* FP:outlives.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_outlives_USE_0002
/* FP:outlives.rs-0004 */ use crate :: rustc_complete :: ty :: { ParamEnvAnd , TyCtxt } ;
/* FP:outlives.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_outlives_USE_0003
/* FP:outlives.rs-0006 */ use crate :: rustc_complete :: Span ;
/* FP:outlives.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_outlives_USE_0004
/* FP:outlives.rs-0008 */ use crate :: infer :: canonical :: { CanonicalQueryInput , CanonicalQueryResponse } ;
/* FP:outlives.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_outlives_USE_0005
/* FP:outlives.rs-0010 */ use crate :: traits :: ObligationCtxt ;
/* FP:outlives.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_outlives_USE_0006
/* FP:outlives.rs-0012 */ use crate :: traits :: query :: dropck_outlives :: { compute_dropck_outlives_inner , trivial_dropck_outlives , } ;
/* FP:outlives.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_outlives_USE_0007
/* FP:outlives.rs-0014 */ use crate :: traits :: query :: type_op :: DropckOutlives ;
/* FP:outlives.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_traits_query_type_op_outlives_IMPL_0008
/* FP:outlives.rs-0016 */ impl < 'tcx > super :: QueryTypeOp < 'tcx > for DropckOutlives < 'tcx > { type QueryResponse = DropckOutlivesResult < 'tcx > ; fn try_fast_path (tcx : TyCtxt < 'tcx > , key : & ParamEnvAnd < 'tcx , Self > ,) -> Option < Self :: QueryResponse > { trivial_dropck_outlives (tcx , key . value . dropped_ty) . then (DropckOutlivesResult :: default) } fn perform_query (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Self > > ,) -> Result < CanonicalQueryResponse < 'tcx , Self :: QueryResponse > , NoSolution > { tcx . dropck_outlives (canonicalized) } fn perform_locally_with_next_solver (ocx : & ObligationCtxt < '_ , 'tcx > , key : ParamEnvAnd < 'tcx , Self > , span : Span ,) -> Result < Self :: QueryResponse , NoSolution > { compute_dropck_outlives_inner (ocx , key . param_env . and (key . value) , span) } }