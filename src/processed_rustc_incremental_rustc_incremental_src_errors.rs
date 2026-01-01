/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_USE_0001
/* FP:errors.rs-0002 */ use std :: path :: { Path , PathBuf } ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_USE_0002
/* FP:errors.rs-0004 */ use rustc_macros :: Diagnostic ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_USE_0003
/* FP:errors.rs-0006 */ use crate :: rustc_complete :: { Ident , Span , Symbol } ;
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0004
/* FP:errors.rs-0008 */ # [derive (Diagnostic)] # [diag (incremental_unrecognized_depnode)] pub (crate) struct UnrecognizedDepNode { # [primary_span] pub span : Span , pub name : Symbol , }
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0005
/* FP:errors.rs-0010 */ # [derive (Diagnostic)] # [diag (incremental_missing_depnode)] pub (crate) struct MissingDepNode { # [primary_span] pub span : Span , }
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0006
/* FP:errors.rs-0012 */ # [derive (Diagnostic)] # [diag (incremental_missing_if_this_changed)] pub (crate) struct MissingIfThisChanged { # [primary_span] pub span : Span , }
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0007
/* FP:errors.rs-0014 */ # [derive (Diagnostic)] # [diag (incremental_ok)] pub (crate) struct Ok { # [primary_span] pub span : Span , }
/* FP:errors.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0008
/* FP:errors.rs-0016 */ # [derive (Diagnostic)] # [diag (incremental_no_path)] pub (crate) struct NoPath { # [primary_span] pub span : Span , pub target : Symbol , pub source : String , }
/* FP:errors.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0009
/* FP:errors.rs-0018 */ # [derive (Diagnostic)] # [diag (incremental_assertion_auto)] pub (crate) struct AssertionAuto < 'a > { # [primary_span] pub span : Span , pub name : & 'a str , pub e : & 'a str , }
/* FP:errors.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0010
/* FP:errors.rs-0020 */ # [derive (Diagnostic)] # [diag (incremental_undefined_clean_dirty_assertions_item)] pub (crate) struct UndefinedCleanDirtyItem { # [primary_span] pub span : Span , pub kind : String , }
/* FP:errors.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0011
/* FP:errors.rs-0022 */ # [derive (Diagnostic)] # [diag (incremental_undefined_clean_dirty_assertions)] pub (crate) struct UndefinedCleanDirty { # [primary_span] pub span : Span , pub kind : String , }
/* FP:errors.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0012
/* FP:errors.rs-0024 */ # [derive (Diagnostic)] # [diag (incremental_repeated_depnode_label)] pub (crate) struct RepeatedDepNodeLabel < 'a > { # [primary_span] pub span : Span , pub label : & 'a str , }
/* FP:errors.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0013
/* FP:errors.rs-0026 */ # [derive (Diagnostic)] # [diag (incremental_unrecognized_depnode_label)] pub (crate) struct UnrecognizedDepNodeLabel < 'a > { # [primary_span] pub span : Span , pub label : & 'a str , }
/* FP:errors.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0014
/* FP:errors.rs-0028 */ # [derive (Diagnostic)] # [diag (incremental_not_dirty)] pub (crate) struct NotDirty < 'a > { # [primary_span] pub span : Span , pub dep_node_str : & 'a str , }
/* FP:errors.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0015
/* FP:errors.rs-0030 */ # [derive (Diagnostic)] # [diag (incremental_not_clean)] pub (crate) struct NotClean < 'a > { # [primary_span] pub span : Span , pub dep_node_str : & 'a str , }
/* FP:errors.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0016
/* FP:errors.rs-0032 */ # [derive (Diagnostic)] # [diag (incremental_not_loaded)] pub (crate) struct NotLoaded < 'a > { # [primary_span] pub span : Span , pub dep_node_str : & 'a str , }
/* FP:errors.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0017
/* FP:errors.rs-0034 */ # [derive (Diagnostic)] # [diag (incremental_unknown_rustc_clean_argument)] pub (crate) struct UnknownRustcCleanArgument { # [primary_span] pub span : Span , }
/* FP:errors.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0018
/* FP:errors.rs-0036 */ # [derive (Diagnostic)] # [diag (incremental_no_cfg)] pub (crate) struct NoCfg { # [primary_span] pub span : Span , }
/* FP:errors.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0019
/* FP:errors.rs-0038 */ # [derive (Diagnostic)] # [diag (incremental_associated_value_expected_for)] pub (crate) struct AssociatedValueExpectedFor { # [primary_span] pub span : Span , pub ident : Ident , }
/* FP:errors.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0020
/* FP:errors.rs-0040 */ # [derive (Diagnostic)] # [diag (incremental_associated_value_expected)] pub (crate) struct AssociatedValueExpected { # [primary_span] pub span : Span , }
/* FP:errors.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0021
/* FP:errors.rs-0042 */ # [derive (Diagnostic)] # [diag (incremental_unchecked_clean)] pub (crate) struct UncheckedClean { # [primary_span] pub span : Span , }
/* FP:errors.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0022
/* FP:errors.rs-0044 */ # [derive (Diagnostic)] # [diag (incremental_delete_old)] pub (crate) struct DeleteOld < 'a > { pub name : & 'a str , pub path : PathBuf , pub err : std :: io :: Error , }
/* FP:errors.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0023
/* FP:errors.rs-0046 */ # [derive (Diagnostic)] # [diag (incremental_create_new)] pub (crate) struct CreateNew < 'a > { pub name : & 'a str , pub path : PathBuf , pub err : std :: io :: Error , }
/* FP:errors.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0024
/* FP:errors.rs-0048 */ # [derive (Diagnostic)] # [diag (incremental_write_new)] pub (crate) struct WriteNew < 'a > { pub name : & 'a str , pub path : PathBuf , pub err : std :: io :: Error , }
/* FP:errors.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0025
/* FP:errors.rs-0050 */ # [derive (Diagnostic)] # [diag (incremental_canonicalize_path)] pub (crate) struct CanonicalizePath { pub path : PathBuf , pub err : std :: io :: Error , }
/* FP:errors.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0026
/* FP:errors.rs-0052 */ # [derive (Diagnostic)] # [diag (incremental_create_incr_comp_dir)] pub (crate) struct CreateIncrCompDir < 'a > { pub tag : & 'a str , pub path : & 'a Path , pub err : std :: io :: Error , }
/* FP:errors.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0027
/* FP:errors.rs-0054 */ # [derive (Diagnostic)] # [diag (incremental_create_lock)] pub (crate) struct CreateLock < 'a > { pub lock_err : std :: io :: Error , pub session_dir : & 'a Path , # [note (incremental_lock_unsupported)] pub is_unsupported_lock : bool , # [help (incremental_cargo_help_1)] # [help (incremental_cargo_help_2)] pub is_cargo : bool , }
/* FP:errors.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0028
/* FP:errors.rs-0056 */ # [derive (Diagnostic)] # [diag (incremental_delete_lock)] pub (crate) struct DeleteLock < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
/* FP:errors.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0029
/* FP:errors.rs-0058 */ # [derive (Diagnostic)] # [diag (incremental_hard_link_failed)] pub (crate) struct HardLinkFailed < 'a > { pub path : & 'a Path , }
/* FP:errors.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0030
/* FP:errors.rs-0060 */ # [derive (Diagnostic)] # [diag (incremental_delete_partial)] pub (crate) struct DeletePartial < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
/* FP:errors.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0031
/* FP:errors.rs-0062 */ # [derive (Diagnostic)] # [diag (incremental_delete_full)] pub (crate) struct DeleteFull < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
/* FP:errors.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0032
/* FP:errors.rs-0064 */ # [derive (Diagnostic)] # [diag (incremental_finalize)] pub (crate) struct Finalize < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
/* FP:errors.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0033
/* FP:errors.rs-0066 */ # [derive (Diagnostic)] # [diag (incremental_invalid_gc_failed)] pub (crate) struct InvalidGcFailed < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
/* FP:errors.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0034
/* FP:errors.rs-0068 */ # [derive (Diagnostic)] # [diag (incremental_finalized_gc_failed)] pub (crate) struct FinalizedGcFailed < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
/* FP:errors.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0035
/* FP:errors.rs-0070 */ # [derive (Diagnostic)] # [diag (incremental_session_gc_failed)] pub (crate) struct SessionGcFailed < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
/* FP:errors.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0036
/* FP:errors.rs-0072 */ # [derive (Diagnostic)] # [diag (incremental_assert_not_loaded)] pub (crate) struct AssertNotLoaded ;
/* FP:errors.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0037
/* FP:errors.rs-0074 */ # [derive (Diagnostic)] # [diag (incremental_assert_loaded)] pub (crate) struct AssertLoaded ;
/* FP:errors.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0038
/* FP:errors.rs-0076 */ # [derive (Diagnostic)] # [diag (incremental_delete_incompatible)] pub (crate) struct DeleteIncompatible { pub path : PathBuf , pub err : std :: io :: Error , }
/* FP:errors.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0039
/* FP:errors.rs-0078 */ # [derive (Diagnostic)] # [diag (incremental_load_dep_graph)] pub (crate) struct LoadDepGraph { pub path : PathBuf , pub err : std :: io :: Error , }
/* FP:errors.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0040
/* FP:errors.rs-0080 */ # [derive (Diagnostic)] # [diag (incremental_move_dep_graph)] pub (crate) struct MoveDepGraph < 'a > { pub from : & 'a Path , pub to : & 'a Path , pub err : std :: io :: Error , }
/* FP:errors.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0041
/* FP:errors.rs-0082 */ # [derive (Diagnostic)] # [diag (incremental_create_dep_graph)] pub (crate) struct CreateDepGraph < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
/* FP:errors.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0042
/* FP:errors.rs-0084 */ # [derive (Diagnostic)] # [diag (incremental_copy_workproduct_to_cache)] pub (crate) struct CopyWorkProductToCache < 'a > { pub from : & 'a Path , pub to : & 'a Path , pub err : std :: io :: Error , }
/* FP:errors.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0043
/* FP:errors.rs-0086 */ # [derive (Diagnostic)] # [diag (incremental_delete_workproduct)] pub (crate) struct DeleteWorkProduct < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
/* FP:errors.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_errors_STRUCT_0044
/* FP:errors.rs-0088 */ # [derive (Diagnostic)] # [diag (incremental_corrupt_file)] pub (crate) struct CorruptFile < 'a > { pub path : & 'a Path , }