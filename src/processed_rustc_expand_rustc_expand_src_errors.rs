/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_USE_0001
/* FP:errors.rs-0002 */ use std :: borrow :: Cow ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_USE_0002
/* FP:errors.rs-0004 */ use crate :: rustc_complete :: ast ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_USE_0003
/* FP:errors.rs-0006 */ use crate :: rustc_complete :: codes :: * ;
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_USE_0004
/* FP:errors.rs-0008 */ use crate :: rustc_complete :: limit :: Limit ;
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_USE_0005
/* FP:errors.rs-0010 */ use rustc_macros :: { Diagnostic , Subdiagnostic } ;
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_USE_0006
/* FP:errors.rs-0012 */ use crate :: rustc_complete :: { Ident , MacroRulesNormalizedIdent , Span , Symbol } ;
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0007
/* FP:errors.rs-0014 */ # [derive (Diagnostic)] # [diag (expand_expr_repeat_no_syntax_vars)] pub (crate) struct NoSyntaxVarsExprRepeat { # [primary_span] pub span : Span , }
/* FP:errors.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0008
/* FP:errors.rs-0016 */ # [derive (Diagnostic)] # [diag (expand_must_repeat_once)] pub (crate) struct MustRepeatOnce { # [primary_span] pub span : Span , }
/* FP:errors.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0009
/* FP:errors.rs-0018 */ # [derive (Diagnostic)] # [diag (expand_count_repetition_misplaced)] pub (crate) struct CountRepetitionMisplaced { # [primary_span] pub span : Span , }
/* FP:errors.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0010
/* FP:errors.rs-0020 */ # [derive (Diagnostic)] # [diag (expand_var_still_repeating)] pub (crate) struct VarStillRepeating { # [primary_span] pub span : Span , pub ident : MacroRulesNormalizedIdent , }
/* FP:errors.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0011
/* FP:errors.rs-0022 */ # [derive (Diagnostic)] # [diag (expand_meta_var_dif_seq_matchers)] pub (crate) struct MetaVarsDifSeqMatchers { # [primary_span] pub span : Span , pub msg : String , }
/* FP:errors.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0012
/* FP:errors.rs-0024 */ # [derive (Diagnostic)] # [diag (expand_resolve_relative_path)] pub (crate) struct ResolveRelativePath { # [primary_span] pub span : Span , pub path : String , }
/* FP:errors.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0013
/* FP:errors.rs-0026 */ # [derive (Diagnostic)] # [diag (expand_collapse_debuginfo_illegal)] pub (crate) struct CollapseMacroDebuginfoIllegal { # [primary_span] pub span : Span , }
/* FP:errors.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0014
/* FP:errors.rs-0028 */ # [derive (Diagnostic)] # [diag (expand_macro_const_stability)] pub (crate) struct MacroConstStability { # [primary_span] # [label] pub span : Span , # [label (expand_label2)] pub head_span : Span , }
/* FP:errors.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0015
/* FP:errors.rs-0030 */ # [derive (Diagnostic)] # [diag (expand_macro_body_stability)] pub (crate) struct MacroBodyStability { # [primary_span] # [label] pub span : Span , # [label (expand_label2)] pub head_span : Span , }
/* FP:errors.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0016
/* FP:errors.rs-0032 */ # [derive (Diagnostic)] # [diag (expand_feature_removed , code = E0557)] # [note] pub (crate) struct FeatureRemoved < 'a > { # [primary_span] # [label] pub span : Span , # [subdiagnostic] pub reason : Option < FeatureRemovedReason < 'a > > , pub removed_rustc_version : & 'a str , pub pull_note : String , }
/* FP:errors.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0017
/* FP:errors.rs-0034 */ # [derive (Subdiagnostic)] # [note (expand_reason)] pub (crate) struct FeatureRemovedReason < 'a > { pub reason : & 'a str , }
/* FP:errors.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0018
/* FP:errors.rs-0036 */ # [derive (Diagnostic)] # [diag (expand_feature_not_allowed , code = E0725)] pub (crate) struct FeatureNotAllowed { # [primary_span] pub span : Span , pub name : Symbol , }
/* FP:errors.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0019
/* FP:errors.rs-0038 */ # [derive (Diagnostic)] # [diag (expand_recursion_limit_reached)] # [help] pub (crate) struct RecursionLimitReached { # [primary_span] pub span : Span , pub descr : String , pub suggested_limit : Limit , pub crate_name : Symbol , }
/* FP:errors.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0020
/* FP:errors.rs-0040 */ # [derive (Diagnostic)] # [diag (expand_malformed_feature_attribute , code = E0556)] pub (crate) struct MalformedFeatureAttribute { # [primary_span] pub span : Span , # [subdiagnostic] pub help : MalformedFeatureAttributeHelp , }
/* FP:errors.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_ENUM_0021
/* FP:errors.rs-0042 */ # [derive (Subdiagnostic)] pub (crate) enum MalformedFeatureAttributeHelp { # [label (expand_expected)] Label { # [primary_span] span : Span , } , # [suggestion (expand_expected , code = "{suggestion}" , applicability = "maybe-incorrect")] Suggestion { # [primary_span] span : Span , suggestion : Symbol , } , }
/* FP:errors.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0022
/* FP:errors.rs-0044 */ # [derive (Diagnostic)] # [diag (expand_remove_expr_not_supported)] pub (crate) struct RemoveExprNotSupported { # [primary_span] pub span : Span , }
/* FP:errors.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_ENUM_0023
/* FP:errors.rs-0046 */ # [derive (Diagnostic)] pub (crate) enum InvalidCfg { # [diag (expand_invalid_cfg_no_parens)] NotFollowedByParens { # [primary_span] # [suggestion (expand_invalid_cfg_expected_syntax , code = "cfg(/* predicate */)" , applicability = "has-placeholders")] span : Span , } , # [diag (expand_invalid_cfg_no_predicate)] NoPredicate { # [primary_span] # [suggestion (expand_invalid_cfg_expected_syntax , code = "cfg(/* predicate */)" , applicability = "has-placeholders")] span : Span , } , # [diag (expand_invalid_cfg_multiple_predicates)] MultiplePredicates { # [primary_span] span : Span , } , # [diag (expand_invalid_cfg_predicate_literal)] PredicateLiteral { # [primary_span] span : Span , } , }
/* FP:errors.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0024
/* FP:errors.rs-0048 */ # [derive (Diagnostic)] # [diag (expand_wrong_fragment_kind)] pub (crate) struct WrongFragmentKind < 'a > { # [primary_span] pub span : Span , pub kind : & 'a str , pub name : & 'a ast :: Path , }
/* FP:errors.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0025
/* FP:errors.rs-0050 */ # [derive (Diagnostic)] # [diag (expand_unsupported_key_value)] pub (crate) struct UnsupportedKeyValue { # [primary_span] pub span : Span , }
/* FP:errors.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0026
/* FP:errors.rs-0052 */ # [derive (Diagnostic)] # [diag (expand_incomplete_parse)] # [note] pub (crate) struct IncompleteParse < 'a > { # [primary_span] pub span : Span , pub descr : String , # [label] pub label_span : Span , pub macro_path : & 'a ast :: Path , pub kind_name : & 'a str , # [note (expand_macro_expands_to_match_arm)] pub expands_to_match_arm : bool , # [suggestion (expand_suggestion_add_semi , style = "verbose" , code = ";" , applicability = "maybe-incorrect")] pub add_semicolon : Option < Span > , }
/* FP:errors.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0027
/* FP:errors.rs-0054 */ # [derive (Diagnostic)] # [diag (expand_remove_node_not_supported)] pub (crate) struct RemoveNodeNotSupported { # [primary_span] pub span : Span , pub descr : & 'static str , }
/* FP:errors.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0028
/* FP:errors.rs-0056 */ # [derive (Diagnostic)] # [diag (expand_module_circular)] pub (crate) struct ModuleCircular { # [primary_span] pub span : Span , pub modules : String , }
/* FP:errors.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0029
/* FP:errors.rs-0058 */ # [derive (Diagnostic)] # [diag (expand_module_in_block)] pub (crate) struct ModuleInBlock { # [primary_span] pub span : Span , # [subdiagnostic] pub name : Option < ModuleInBlockName > , }
/* FP:errors.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0030
/* FP:errors.rs-0060 */ # [derive (Subdiagnostic)] # [note (expand_note)] pub (crate) struct ModuleInBlockName { # [primary_span] pub span : Span , pub name : Ident , }
/* FP:errors.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0031
/* FP:errors.rs-0062 */ # [derive (Diagnostic)] # [diag (expand_module_file_not_found , code = E0583)] # [help] # [note] pub (crate) struct ModuleFileNotFound { # [primary_span] pub span : Span , pub name : Ident , pub default_path : String , pub secondary_path : String , }
/* FP:errors.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0032
/* FP:errors.rs-0064 */ # [derive (Diagnostic)] # [diag (expand_module_multiple_candidates , code = E0761)] # [help] pub (crate) struct ModuleMultipleCandidates { # [primary_span] pub span : Span , pub name : Ident , pub default_path : String , pub secondary_path : String , }
/* FP:errors.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0033
/* FP:errors.rs-0066 */ # [derive (Diagnostic)] # [diag (expand_trace_macro)] pub (crate) struct TraceMacro { # [primary_span] pub span : Span , }
/* FP:errors.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0034
/* FP:errors.rs-0068 */ # [derive (Diagnostic)] # [diag (expand_proc_macro_panicked)] pub (crate) struct ProcMacroPanicked { # [primary_span] pub span : Span , # [subdiagnostic] pub message : Option < ProcMacroPanickedHelp > , }
/* FP:errors.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0035
/* FP:errors.rs-0070 */ # [derive (Subdiagnostic)] # [help (expand_help)] pub (crate) struct ProcMacroPanickedHelp { pub message : String , }
/* FP:errors.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0036
/* FP:errors.rs-0072 */ # [derive (Diagnostic)] # [diag (expand_proc_macro_derive_panicked)] pub (crate) struct ProcMacroDerivePanicked { # [primary_span] pub span : Span , # [subdiagnostic] pub message : Option < ProcMacroDerivePanickedHelp > , }
/* FP:errors.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0037
/* FP:errors.rs-0074 */ # [derive (Subdiagnostic)] # [help (expand_help)] pub (crate) struct ProcMacroDerivePanickedHelp { pub message : String , }
/* FP:errors.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0038
/* FP:errors.rs-0076 */ # [derive (Diagnostic)] # [diag (expand_custom_attribute_panicked)] pub (crate) struct CustomAttributePanicked { # [primary_span] pub span : Span , # [subdiagnostic] pub message : Option < CustomAttributePanickedHelp > , }
/* FP:errors.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0039
/* FP:errors.rs-0078 */ # [derive (Subdiagnostic)] # [help (expand_help)] pub (crate) struct CustomAttributePanickedHelp { pub message : String , }
/* FP:errors.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0040
/* FP:errors.rs-0080 */ # [derive (Diagnostic)] # [diag (expand_proc_macro_derive_tokens)] pub (crate) struct ProcMacroDeriveTokens { # [primary_span] pub span : Span , }
/* FP:errors.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0041
/* FP:errors.rs-0082 */ # [derive (Diagnostic)] # [diag (expand_duplicate_matcher_binding)] pub (crate) struct DuplicateMatcherBinding { # [primary_span] # [label] pub span : Span , # [label (expand_label2)] pub prev : Span , }
/* FP:errors.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0042
/* FP:errors.rs-0084 */ # [derive (Diagnostic)] # [diag (expand_missing_fragment_specifier)] # [note] # [help (expand_valid)] pub (crate) struct MissingFragmentSpecifier { # [primary_span] pub span : Span , # [suggestion (expand_suggestion_add_fragspec , style = "verbose" , code = ":spec" , applicability = "maybe-incorrect")] pub add_span : Span , pub valid : & 'static str , }
/* FP:errors.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0043
/* FP:errors.rs-0086 */ # [derive (Diagnostic)] # [diag (expand_invalid_fragment_specifier)] # [help] pub (crate) struct InvalidFragmentSpecifier { # [primary_span] pub span : Span , pub fragment : Ident , pub help : & 'static str , }
/* FP:errors.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0044
/* FP:errors.rs-0088 */ # [derive (Diagnostic)] # [diag (expand_expected_paren_or_brace)] pub (crate) struct ExpectedParenOrBrace < 'a > { # [primary_span] pub span : Span , pub token : Cow < 'a , str > , }
/* FP:errors.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0045
/* FP:errors.rs-0090 */ # [derive (Diagnostic)] # [diag (expand_empty_delegation_mac)] pub (crate) struct EmptyDelegationMac { # [primary_span] pub span : Span , pub kind : String , }
/* FP:errors.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0046
/* FP:errors.rs-0092 */ # [derive (Diagnostic)] # [diag (expand_glob_delegation_outside_impls)] pub (crate) struct GlobDelegationOutsideImpls { # [primary_span] pub span : Span , }
/* FP:errors.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0047
/* FP:errors.rs-0094 */ # [derive (Diagnostic)] # [diag (expand_crate_name_in_cfg_attr)] pub (crate) struct CrateNameInCfgAttr { # [primary_span] pub span : Span , }
/* FP:errors.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0048
/* FP:errors.rs-0096 */ # [derive (Diagnostic)] # [diag (expand_crate_type_in_cfg_attr)] pub (crate) struct CrateTypeInCfgAttr { # [primary_span] pub span : Span , }
/* FP:errors.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0049
/* FP:errors.rs-0098 */ # [derive (Diagnostic)] # [diag (expand_glob_delegation_traitless_qpath)] pub (crate) struct GlobDelegationTraitlessQpath { # [primary_span] pub span : Span , }
/* FP:errors.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0050
/* FP:errors.rs-0100 */ # [derive (Diagnostic)] # [diag (expand_proc_macro_back_compat)] # [note] pub (crate) struct ProcMacroBackCompat { pub crate_name : String , pub fixed_version : String , }
/* FP:errors.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_USE_0051
/* FP:errors.rs-0102 */ pub (crate) use metavar_exprs :: * ;
/* FP:errors.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_MOD_0052
/* FP:errors.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0053
/* FP:errors.rs-0106 */ # [derive (Diagnostic)] # [diag (expand_macro_args_bad_delim)] pub (crate) struct MacroArgsBadDelim { # [primary_span] pub span : Span , # [subdiagnostic] pub sugg : MacroArgsBadDelimSugg , pub rule_kw : Symbol , }
/* FP:errors.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_errors_STRUCT_0054
/* FP:errors.rs-0108 */ # [derive (Subdiagnostic)] # [multipart_suggestion (expand_macro_args_bad_delim_sugg , applicability = "machine-applicable")] pub (crate) struct MacroArgsBadDelimSugg { # [suggestion_part (code = "(")] pub open : Span , # [suggestion_part (code = ")")] pub close : Span , }