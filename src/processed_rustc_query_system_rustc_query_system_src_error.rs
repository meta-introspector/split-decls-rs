/* FP:error.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_error_USE_0001
/* FP:error.rs-0002 */ use crate :: rustc_complete :: codes :: * ;
/* FP:error.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_error_USE_0002
/* FP:error.rs-0004 */ use crate :: rustc_complete :: limit :: Limit ;
/* FP:error.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_error_USE_0003
/* FP:error.rs-0006 */ use rustc_macros :: { Diagnostic , Subdiagnostic } ;
/* FP:error.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_error_USE_0004
/* FP:error.rs-0008 */ use crate :: rustc_complete :: { Span , Symbol } ;
/* FP:error.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_error_STRUCT_0005
/* FP:error.rs-0010 */ # [derive (Subdiagnostic)] # [note (query_system_cycle_stack_middle)] pub (crate) struct CycleStack { # [primary_span] pub span : Span , pub desc : String , }
/* FP:error.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_error_ENUM_0006
/* FP:error.rs-0012 */ # [derive (Copy , Clone)] pub enum HandleCycleError { Error , Fatal , DelayBug , Stash , }
/* FP:error.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_error_ENUM_0007
/* FP:error.rs-0014 */ # [derive (Subdiagnostic)] pub (crate) enum StackCount { # [note (query_system_cycle_stack_single)] Single , # [note (query_system_cycle_stack_multiple)] Multiple , }
/* FP:error.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_error_ENUM_0008
/* FP:error.rs-0016 */ # [derive (Subdiagnostic)] pub (crate) enum Alias { # [note (query_system_cycle_recursive_ty_alias)] # [help (query_system_cycle_recursive_ty_alias_help1)] # [help (query_system_cycle_recursive_ty_alias_help2)] Ty , # [note (query_system_cycle_recursive_trait_alias)] Trait , }
/* FP:error.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_error_STRUCT_0009
/* FP:error.rs-0018 */ # [derive (Subdiagnostic)] # [note (query_system_cycle_usage)] pub (crate) struct CycleUsage { # [primary_span] pub span : Span , pub usage : String , }
/* FP:error.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_error_STRUCT_0010
/* FP:error.rs-0020 */ # [derive (Diagnostic)] # [diag (query_system_cycle , code = E0391)] pub (crate) struct Cycle { # [primary_span] pub span : Span , pub stack_bottom : String , # [subdiagnostic] pub cycle_stack : Vec < CycleStack > , # [subdiagnostic] pub stack_count : StackCount , # [subdiagnostic] pub alias : Option < Alias > , # [subdiagnostic] pub cycle_usage : Option < CycleUsage > , # [note] pub note_span : () , }
/* FP:error.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_error_STRUCT_0011
/* FP:error.rs-0022 */ # [derive (Diagnostic)] # [diag (query_system_reentrant)] pub (crate) struct Reentrant ;
/* FP:error.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_error_STRUCT_0012
/* FP:error.rs-0024 */ # [derive (Diagnostic)] # [diag (query_system_increment_compilation)] # [help] # [note (query_system_increment_compilation_note1)] # [note (query_system_increment_compilation_note2)] pub (crate) struct IncrementCompilation { pub run_cmd : String , pub dep_node : String , }
/* FP:error.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_error_STRUCT_0013
/* FP:error.rs-0026 */ # [derive (Diagnostic)] # [help] # [diag (query_system_query_overflow)] pub struct QueryOverflow { # [primary_span] pub span : Span , # [subdiagnostic] pub note : QueryOverflowNote , pub suggested_limit : Limit , pub crate_name : Symbol , }
/* FP:error.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_error_STRUCT_0014
/* FP:error.rs-0028 */ # [derive (Subdiagnostic)] # [note (query_system_overflow_note)] pub struct QueryOverflowNote { pub desc : String , pub depth : usize , }