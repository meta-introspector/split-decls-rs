/* FP:anon_const.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_next_trait_solver_src_solve_normalizes_to_anon_const_USE_0001
/* FP:anon_const.rs-0002 */ use rustc_type_ir :: { self as ty , Interner } ;
/* FP:anon_const.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_next_trait_solver_src_solve_normalizes_to_anon_const_USE_0002
/* FP:anon_const.rs-0004 */ use tracing :: instrument ;
/* FP:anon_const.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_next_trait_solver_src_solve_normalizes_to_anon_const_USE_0003
/* FP:anon_const.rs-0006 */ use crate :: delegate :: SolverDelegate ;
/* FP:anon_const.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_next_trait_solver_src_solve_normalizes_to_anon_const_USE_0004
/* FP:anon_const.rs-0008 */ use crate :: solve :: { Certainty , EvalCtxt , Goal , QueryResult } ;
/* FP:anon_const.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_next_trait_solver_src_solve_normalizes_to_anon_const_IMPL_0005
/* FP:anon_const.rs-0010 */ impl < D , I > EvalCtxt < '_ , D > where D : SolverDelegate < Interner = I > , I : Interner , { # [instrument (level = "trace" , skip (self) , ret)] pub (super) fn normalize_anon_const (& mut self , goal : Goal < I , ty :: NormalizesTo < I > > ,) -> QueryResult < I > { if let Some (normalized_const) = self . evaluate_const (goal . param_env , ty :: UnevaluatedConst :: new (goal . predicate . alias . def_id , goal . predicate . alias . args) ,) { self . instantiate_normalizes_to_term (goal , normalized_const . into ()) ; self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } else { self . evaluate_added_goals_and_make_canonical_response (Certainty :: AMBIGUOUS) } } }