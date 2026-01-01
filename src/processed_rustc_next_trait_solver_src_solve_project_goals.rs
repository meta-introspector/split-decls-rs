/* FP:project_goals.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_next_trait_solver_src_solve_project_goals_USE_0001
/* FP:project_goals.rs-0002 */ use rustc_type_ir :: { self as ty , Interner , ProjectionPredicate } ;
/* FP:project_goals.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_next_trait_solver_src_solve_project_goals_USE_0002
/* FP:project_goals.rs-0004 */ use tracing :: instrument ;
/* FP:project_goals.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_next_trait_solver_src_solve_project_goals_USE_0003
/* FP:project_goals.rs-0006 */ use crate :: delegate :: SolverDelegate ;
/* FP:project_goals.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_next_trait_solver_src_solve_project_goals_USE_0004
/* FP:project_goals.rs-0008 */ use crate :: solve :: { Certainty , EvalCtxt , Goal , GoalSource , QueryResult } ;
/* FP:project_goals.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_next_trait_solver_src_solve_project_goals_IMPL_0005
/* FP:project_goals.rs-0010 */ impl < D , I > EvalCtxt < '_ , D > where D : SolverDelegate < Interner = I > , I : Interner , { # [instrument (level = "trace" , skip (self) , ret)] pub (super) fn compute_projection_goal (& mut self , goal : Goal < I , ProjectionPredicate < I > > ,) -> QueryResult < I > { let cx = self . cx () ; let projection_term = goal . predicate . projection_term . to_term (cx) ; let goal = goal . with (cx , ty :: PredicateKind :: AliasRelate (projection_term , goal . predicate . term , ty :: AliasRelationDirection :: Equate ,) ,) ; self . add_goal (GoalSource :: TypeRelating , goal) ; self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } }