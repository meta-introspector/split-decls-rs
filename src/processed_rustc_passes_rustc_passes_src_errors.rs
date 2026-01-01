/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_USE_0001
/* FP:errors.rs-0002 */ use std :: io :: Error ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_USE_0002
/* FP:errors.rs-0004 */ use std :: path :: { Path , PathBuf } ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_USE_0003
/* FP:errors.rs-0006 */ use crate :: rustc_complete :: codes :: * ;
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_USE_0004
/* FP:errors.rs-0008 */ use crate :: rustc_complete :: { Applicability , Diag , DiagCtxtHandle , DiagSymbolList , Diagnostic , EmissionGuarantee , Level , MultiSpan , Subdiagnostic , } ;
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_USE_0005
/* FP:errors.rs-0010 */ use crate :: rustc_complete :: Target ;
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_USE_0006
/* FP:errors.rs-0012 */ use crate :: rustc_complete :: attrs :: { MirDialect , MirPhase } ;
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_USE_0007
/* FP:errors.rs-0014 */ use rustc_macros :: { Diagnostic , LintDiagnostic , Subdiagnostic } ;
/* FP:errors.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_USE_0008
/* FP:errors.rs-0016 */ use crate :: rustc_complete :: ty :: { MainDefinition , Ty } ;
/* FP:errors.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_USE_0009
/* FP:errors.rs-0018 */ use crate :: rustc_complete :: { DUMMY_SP , Span , Symbol } ;
/* FP:errors.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_USE_0010
/* FP:errors.rs-0020 */ use crate :: check_attr :: ProcMacroKind ;
/* FP:errors.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_USE_0011
/* FP:errors.rs-0022 */ use crate :: fluent_generated as fluent ;
/* FP:errors.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_USE_0012
/* FP:errors.rs-0024 */ use crate :: lang_items :: Duplicate ;
/* FP:errors.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0013
/* FP:errors.rs-0026 */ # [derive (LintDiagnostic)] # [diag (passes_incorrect_do_not_recommend_location)] pub (crate) struct IncorrectDoNotRecommendLocation ;
/* FP:errors.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0014
/* FP:errors.rs-0028 */ # [derive (LintDiagnostic)] # [diag (passes_incorrect_do_not_recommend_args)] pub (crate) struct DoNotRecommendDoesNotExpectArgs ;
/* FP:errors.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0015
/* FP:errors.rs-0030 */ # [derive (Diagnostic)] # [diag (passes_autodiff_attr)] pub (crate) struct AutoDiffAttr { # [primary_span] # [label] pub attr_span : Span , }
/* FP:errors.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0016
/* FP:errors.rs-0032 */ # [derive (Diagnostic)] # [diag (passes_loop_match_attr)] pub (crate) struct LoopMatchAttr { # [primary_span] pub attr_span : Span , # [label] pub node_span : Span , }
/* FP:errors.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0017
/* FP:errors.rs-0034 */ # [derive (Diagnostic)] # [diag (passes_const_continue_attr)] pub (crate) struct ConstContinueAttr { # [primary_span] pub attr_span : Span , # [label] pub node_span : Span , }
/* FP:errors.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0018
/* FP:errors.rs-0036 */ # [derive (LintDiagnostic)] # [diag (passes_mixed_export_name_and_no_mangle)] pub (crate) struct MixedExportNameAndNoMangle { # [label] # [suggestion (style = "verbose" , code = "" , applicability = "machine-applicable")] pub no_mangle_span : Span , # [note] pub export_name_span : Span , pub no_mangle_attr : & 'static str , pub export_name_attr : & 'static str , }
/* FP:errors.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0019
/* FP:errors.rs-0038 */ # [derive (LintDiagnostic)] # [diag (passes_outer_crate_level_attr)] pub (crate) struct OuterCrateLevelAttr { # [subdiagnostic] pub suggestion : OuterCrateLevelAttrSuggestion , }
/* FP:errors.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0020
/* FP:errors.rs-0040 */ # [derive (Subdiagnostic)] # [multipart_suggestion (passes_outer_crate_level_attr_suggestion , style = "verbose")] pub (crate) struct OuterCrateLevelAttrSuggestion { # [suggestion_part (code = "!")] pub bang_position : Span , }
/* FP:errors.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0021
/* FP:errors.rs-0042 */ # [derive (LintDiagnostic)] # [diag (passes_inner_crate_level_attr)] pub (crate) struct InnerCrateLevelAttr ;
/* FP:errors.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0022
/* FP:errors.rs-0044 */ # [derive (LintDiagnostic)] # [diag (passes_ignored_attr_with_macro)] pub (crate) struct IgnoredAttrWithMacro < 'a > { pub sym : & 'a str , }
/* FP:errors.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0023
/* FP:errors.rs-0046 */ # [derive (Diagnostic)] # [diag (passes_should_be_applied_to_fn)] pub (crate) struct AttrShouldBeAppliedToFn { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , pub on_crate : bool , }
/* FP:errors.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0024
/* FP:errors.rs-0048 */ # [derive (Diagnostic)] # [diag (passes_non_exhaustive_with_default_field_values)] pub (crate) struct NonExhaustiveWithDefaultFieldValues { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , }
/* FP:errors.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0025
/* FP:errors.rs-0050 */ # [derive (Diagnostic)] # [diag (passes_should_be_applied_to_trait)] pub (crate) struct AttrShouldBeAppliedToTrait { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , }
/* FP:errors.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0026
/* FP:errors.rs-0052 */ # [derive (Diagnostic)] # [diag (passes_should_be_applied_to_static)] pub (crate) struct AttrShouldBeAppliedToStatic { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , }
/* FP:errors.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0027
/* FP:errors.rs-0054 */ # [derive (Diagnostic)] # [diag (passes_doc_expect_str)] pub (crate) struct DocExpectStr < 'a > { # [primary_span] pub attr_span : Span , pub attr_name : & 'a str , }
/* FP:errors.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0028
/* FP:errors.rs-0056 */ # [derive (Diagnostic)] # [diag (passes_doc_alias_empty)] pub (crate) struct DocAliasEmpty < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , }
/* FP:errors.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0029
/* FP:errors.rs-0058 */ # [derive (Diagnostic)] # [diag (passes_doc_alias_bad_char)] pub (crate) struct DocAliasBadChar < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , pub char_ : char , }
/* FP:errors.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0030
/* FP:errors.rs-0060 */ # [derive (Diagnostic)] # [diag (passes_doc_alias_start_end)] pub (crate) struct DocAliasStartEnd < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , }
/* FP:errors.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0031
/* FP:errors.rs-0062 */ # [derive (Diagnostic)] # [diag (passes_doc_alias_bad_location)] pub (crate) struct DocAliasBadLocation < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , pub location : & 'a str , }
/* FP:errors.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0032
/* FP:errors.rs-0064 */ # [derive (Diagnostic)] # [diag (passes_doc_alias_not_an_alias)] pub (crate) struct DocAliasNotAnAlias < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , }
/* FP:errors.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0033
/* FP:errors.rs-0066 */ # [derive (LintDiagnostic)] # [diag (passes_doc_alias_duplicated)] pub (crate) struct DocAliasDuplicated { # [label] pub first_defn : Span , }
/* FP:errors.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0034
/* FP:errors.rs-0068 */ # [derive (Diagnostic)] # [diag (passes_doc_alias_not_string_literal)] pub (crate) struct DocAliasNotStringLiteral { # [primary_span] pub span : Span , }
/* FP:errors.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0035
/* FP:errors.rs-0070 */ # [derive (Diagnostic)] # [diag (passes_doc_alias_malformed)] pub (crate) struct DocAliasMalformed { # [primary_span] pub span : Span , }
/* FP:errors.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0036
/* FP:errors.rs-0072 */ # [derive (Diagnostic)] # [diag (passes_doc_keyword_attribute_empty_mod)] pub (crate) struct DocKeywordAttributeEmptyMod { # [primary_span] pub span : Span , pub attr_name : & 'static str , }
/* FP:errors.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0037
/* FP:errors.rs-0074 */ # [derive (Diagnostic)] # [diag (passes_doc_keyword_not_keyword)] # [help] pub (crate) struct DocKeywordNotKeyword { # [primary_span] pub span : Span , pub keyword : Symbol , }
/* FP:errors.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0038
/* FP:errors.rs-0076 */ # [derive (Diagnostic)] # [diag (passes_doc_attribute_not_attribute)] # [help] pub (crate) struct DocAttributeNotAttribute { # [primary_span] pub span : Span , pub attribute : Symbol , }
/* FP:errors.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0039
/* FP:errors.rs-0078 */ # [derive (Diagnostic)] # [diag (passes_doc_keyword_attribute_not_mod)] pub (crate) struct DocKeywordAttributeNotMod { # [primary_span] pub span : Span , pub attr_name : & 'static str , }
/* FP:errors.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0040
/* FP:errors.rs-0080 */ # [derive (Diagnostic)] # [diag (passes_doc_fake_variadic_not_valid)] pub (crate) struct DocFakeVariadicNotValid { # [primary_span] pub span : Span , }
/* FP:errors.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0041
/* FP:errors.rs-0082 */ # [derive (Diagnostic)] # [diag (passes_doc_keyword_only_impl)] pub (crate) struct DocKeywordOnlyImpl { # [primary_span] pub span : Span , }
/* FP:errors.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0042
/* FP:errors.rs-0084 */ # [derive (Diagnostic)] # [diag (passes_doc_search_unbox_invalid)] pub (crate) struct DocSearchUnboxInvalid { # [primary_span] pub span : Span , }
/* FP:errors.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0043
/* FP:errors.rs-0086 */ # [derive (Diagnostic)] # [diag (passes_doc_inline_conflict)] # [help] pub (crate) struct DocKeywordConflict { # [primary_span] pub spans : MultiSpan , }
/* FP:errors.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0044
/* FP:errors.rs-0088 */ # [derive (LintDiagnostic)] # [diag (passes_doc_inline_only_use)] # [note] pub (crate) struct DocInlineOnlyUse { # [label] pub attr_span : Span , # [label (passes_not_a_use_item_label)] pub item_span : Option < Span > , }
/* FP:errors.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0045
/* FP:errors.rs-0090 */ # [derive (LintDiagnostic)] # [diag (passes_doc_masked_only_extern_crate)] # [note] pub (crate) struct DocMaskedOnlyExternCrate { # [label] pub attr_span : Span , # [label (passes_not_an_extern_crate_label)] pub item_span : Option < Span > , }
/* FP:errors.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0046
/* FP:errors.rs-0092 */ # [derive (LintDiagnostic)] # [diag (passes_doc_masked_not_extern_crate_self)] pub (crate) struct DocMaskedNotExternCrateSelf { # [label] pub attr_span : Span , # [label (passes_extern_crate_self_label)] pub item_span : Option < Span > , }
/* FP:errors.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0047
/* FP:errors.rs-0094 */ # [derive (Diagnostic)] # [diag (passes_doc_attr_not_crate_level)] pub (crate) struct DocAttrNotCrateLevel < 'a > { # [primary_span] pub span : Span , pub attr_name : & 'a str , }
/* FP:errors.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0048
/* FP:errors.rs-0096 */ # [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown)] pub (crate) struct DocTestUnknown { pub path : String , }
/* FP:errors.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0049
/* FP:errors.rs-0098 */ # [derive (LintDiagnostic)] # [diag (passes_doc_test_literal)] pub (crate) struct DocTestLiteral ;
/* FP:errors.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0050
/* FP:errors.rs-0100 */ # [derive (LintDiagnostic)] # [diag (passes_doc_test_takes_list)] pub (crate) struct DocTestTakesList ;
/* FP:errors.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0051
/* FP:errors.rs-0102 */ # [derive (LintDiagnostic)] # [diag (passes_doc_cfg_hide_takes_list)] pub (crate) struct DocCfgHideTakesList ;
/* FP:errors.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0052
/* FP:errors.rs-0104 */ # [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_any)] pub (crate) struct DocTestUnknownAny { pub path : String , }
/* FP:errors.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0053
/* FP:errors.rs-0106 */ # [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_spotlight)] # [note] # [note (passes_no_op_note)] pub (crate) struct DocTestUnknownSpotlight { pub path : String , # [suggestion (style = "short" , applicability = "machine-applicable" , code = "notable_trait")] pub span : Span , }
/* FP:errors.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0054
/* FP:errors.rs-0108 */ # [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_passes)] # [note] # [help] # [note (passes_no_op_note)] pub (crate) struct DocTestUnknownPasses { pub path : String , # [label] pub span : Span , }
/* FP:errors.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0055
/* FP:errors.rs-0110 */ # [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_plugins)] # [note] # [note (passes_no_op_note)] pub (crate) struct DocTestUnknownPlugins { pub path : String , # [label] pub span : Span , }
/* FP:errors.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0056
/* FP:errors.rs-0112 */ # [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_include)] pub (crate) struct DocTestUnknownInclude { pub path : String , pub value : String , pub inner : & 'static str , # [suggestion (code = "#{inner}[doc = include_str!(\"{value}\")]")] pub sugg : (Span , Applicability) , }
/* FP:errors.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0057
/* FP:errors.rs-0114 */ # [derive (LintDiagnostic)] # [diag (passes_doc_invalid)] pub (crate) struct DocInvalid ;
/* FP:errors.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0058
/* FP:errors.rs-0116 */ # [derive (Diagnostic)] # [diag (passes_has_incoherent_inherent_impl)] pub (crate) struct HasIncoherentInherentImpl { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
/* FP:errors.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0059
/* FP:errors.rs-0118 */ # [derive (Diagnostic)] # [diag (passes_both_ffi_const_and_pure , code = E0757)] pub (crate) struct BothFfiConstAndPure { # [primary_span] pub attr_span : Span , }
/* FP:errors.rs-0119 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0060
/* FP:errors.rs-0120 */ # [derive (Diagnostic)] # [diag (passes_must_not_suspend)] pub (crate) struct MustNotSuspend { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
/* FP:errors.rs-0121 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0061
/* FP:errors.rs-0122 */ # [derive (LintDiagnostic)] # [diag (passes_link)] # [warning] pub (crate) struct Link { # [label] pub span : Option < Span > , }
/* FP:errors.rs-0123 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0062
/* FP:errors.rs-0124 */ # [derive (Diagnostic)] # [diag (passes_no_link)] pub (crate) struct NoLink { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
/* FP:errors.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0063
/* FP:errors.rs-0126 */ # [derive (Diagnostic)] # [diag (passes_rustc_legacy_const_generics_only)] pub (crate) struct RustcLegacyConstGenericsOnly { # [primary_span] pub attr_span : Span , # [label] pub param_span : Span , }
/* FP:errors.rs-0127 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0064
/* FP:errors.rs-0128 */ # [derive (Diagnostic)] # [diag (passes_rustc_legacy_const_generics_index)] pub (crate) struct RustcLegacyConstGenericsIndex { # [primary_span] pub attr_span : Span , # [label] pub generics_span : Span , }
/* FP:errors.rs-0129 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0065
/* FP:errors.rs-0130 */ # [derive (Diagnostic)] # [diag (passes_rustc_legacy_const_generics_index_exceed)] pub (crate) struct RustcLegacyConstGenericsIndexExceed { # [primary_span] # [label] pub span : Span , pub arg_count : usize , }
/* FP:errors.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0066
/* FP:errors.rs-0132 */ # [derive (Diagnostic)] # [diag (passes_rustc_legacy_const_generics_index_negative)] pub (crate) struct RustcLegacyConstGenericsIndexNegative { # [primary_span] pub invalid_args : Vec < Span > , }
/* FP:errors.rs-0133 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0067
/* FP:errors.rs-0134 */ # [derive (Diagnostic)] # [diag (passes_rustc_dirty_clean)] pub (crate) struct RustcDirtyClean { # [primary_span] pub span : Span , }
/* FP:errors.rs-0135 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0068
/* FP:errors.rs-0136 */ # [derive (Diagnostic)] # [diag (passes_repr_conflicting , code = E0566)] pub (crate) struct ReprConflicting { # [primary_span] pub hint_spans : Vec < Span > , }
/* FP:errors.rs-0137 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0069
/* FP:errors.rs-0138 */ # [derive (Diagnostic)] # [diag (passes_repr_align_greater_than_target_max , code = E0589)] # [note] pub (crate) struct InvalidReprAlignForTarget { # [primary_span] pub span : Span , pub size : u64 , }
/* FP:errors.rs-0139 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0070
/* FP:errors.rs-0140 */ # [derive (LintDiagnostic)] # [diag (passes_repr_conflicting , code = E0566)] pub (crate) struct ReprConflictingLint ;
/* FP:errors.rs-0141 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0071
/* FP:errors.rs-0142 */ # [derive (Diagnostic)] # [diag (passes_macro_only_attribute)] pub (crate) struct MacroOnlyAttribute { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
/* FP:errors.rs-0143 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0072
/* FP:errors.rs-0144 */ # [derive (Diagnostic)] # [diag (passes_debug_visualizer_placement)] pub (crate) struct DebugVisualizerPlacement { # [primary_span] pub span : Span , }
/* FP:errors.rs-0145 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0073
/* FP:errors.rs-0146 */ # [derive (Diagnostic)] # [diag (passes_debug_visualizer_invalid)] # [note (passes_note_1)] # [note (passes_note_2)] # [note (passes_note_3)] pub (crate) struct DebugVisualizerInvalid { # [primary_span] pub span : Span , }
/* FP:errors.rs-0147 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0074
/* FP:errors.rs-0148 */ # [derive (Diagnostic)] # [diag (passes_debug_visualizer_unreadable)] pub (crate) struct DebugVisualizerUnreadable < 'a > { # [primary_span] pub span : Span , pub file : & 'a Path , pub error : Error , }
/* FP:errors.rs-0149 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0075
/* FP:errors.rs-0150 */ # [derive (Diagnostic)] # [diag (passes_rustc_allow_const_fn_unstable)] pub (crate) struct RustcAllowConstFnUnstable { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
/* FP:errors.rs-0151 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0076
/* FP:errors.rs-0152 */ # [derive (Diagnostic)] # [diag (passes_rustc_pub_transparent)] pub (crate) struct RustcPubTransparent { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
/* FP:errors.rs-0153 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0077
/* FP:errors.rs-0154 */ # [derive (Diagnostic)] # [diag (passes_rustc_force_inline_coro)] pub (crate) struct RustcForceInlineCoro { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
/* FP:errors.rs-0155 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_ENUM_0078
/* FP:errors.rs-0156 */ # [derive (LintDiagnostic)] pub (crate) enum MacroExport { # [diag (passes_macro_export)] Normal , # [diag (passes_macro_export_on_decl_macro)] # [note] OnDeclMacro , # [diag (passes_invalid_macro_export_arguments)] InvalidArgument , # [diag (passes_invalid_macro_export_arguments_too_many_items)] TooManyItems , }
/* FP:errors.rs-0157 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_ENUM_0079
/* FP:errors.rs-0158 */ # [derive (Subdiagnostic)] pub (crate) enum UnusedNote { # [note (passes_unused_empty_lints_note)] EmptyList { name : Symbol } , # [note (passes_unused_no_lints_note)] NoLints { name : Symbol } , # [note (passes_unused_default_method_body_const_note)] DefaultMethodBodyConst , # [note (passes_unused_linker_messages_note)] LinkerMessagesBinaryCrateOnly , }
/* FP:errors.rs-0159 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0080
/* FP:errors.rs-0160 */ # [derive (LintDiagnostic)] # [diag (passes_unused)] pub (crate) struct Unused { # [suggestion (code = "" , applicability = "machine-applicable")] pub attr_span : Span , # [subdiagnostic] pub note : UnusedNote , }
/* FP:errors.rs-0161 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0081
/* FP:errors.rs-0162 */ # [derive (Diagnostic)] # [diag (passes_non_exported_macro_invalid_attrs , code = E0518)] pub (crate) struct NonExportedMacroInvalidAttrs { # [primary_span] # [label] pub attr_span : Span , }
/* FP:errors.rs-0163 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0082
/* FP:errors.rs-0164 */ # [derive (Diagnostic)] # [diag (passes_may_dangle)] pub (crate) struct InvalidMayDangle { # [primary_span] pub attr_span : Span , }
/* FP:errors.rs-0165 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0083
/* FP:errors.rs-0166 */ # [derive (LintDiagnostic)] # [diag (passes_unused_duplicate)] pub (crate) struct UnusedDuplicate { # [suggestion (code = "" , applicability = "machine-applicable")] pub this : Span , # [note] pub other : Span , # [warning] pub warning : bool , }
/* FP:errors.rs-0167 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0084
/* FP:errors.rs-0168 */ # [derive (Diagnostic)] # [diag (passes_unused_multiple)] pub (crate) struct UnusedMultiple { # [primary_span] # [suggestion (code = "" , applicability = "machine-applicable")] pub this : Span , # [note] pub other : Span , pub name : Symbol , }
/* FP:errors.rs-0169 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0085
/* FP:errors.rs-0170 */ # [derive (Diagnostic)] # [diag (passes_rustc_lint_opt_ty)] pub (crate) struct RustcLintOptTy { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
/* FP:errors.rs-0171 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0086
/* FP:errors.rs-0172 */ # [derive (Diagnostic)] # [diag (passes_rustc_lint_opt_deny_field_access)] pub (crate) struct RustcLintOptDenyFieldAccess { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
/* FP:errors.rs-0173 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0087
/* FP:errors.rs-0174 */ # [derive (Diagnostic)] # [diag (passes_collapse_debuginfo)] pub (crate) struct CollapseDebuginfo { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , }
/* FP:errors.rs-0175 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0088
/* FP:errors.rs-0176 */ # [derive (LintDiagnostic)] # [diag (passes_deprecated_annotation_has_no_effect)] pub (crate) struct DeprecatedAnnotationHasNoEffect { # [suggestion (applicability = "machine-applicable" , code = "")] pub span : Span , }
/* FP:errors.rs-0177 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0089
/* FP:errors.rs-0178 */ # [derive (Diagnostic)] # [diag (passes_unknown_external_lang_item , code = E0264)] pub (crate) struct UnknownExternLangItem { # [primary_span] pub span : Span , pub lang_item : Symbol , }
/* FP:errors.rs-0179 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0090
/* FP:errors.rs-0180 */ # [derive (Diagnostic)] # [diag (passes_missing_panic_handler)] pub (crate) struct MissingPanicHandler ;
/* FP:errors.rs-0181 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0091
/* FP:errors.rs-0182 */ # [derive (Diagnostic)] # [diag (passes_panic_unwind_without_std)] # [help] # [note] pub (crate) struct PanicUnwindWithoutStd ;
/* FP:errors.rs-0183 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0092
/* FP:errors.rs-0184 */ # [derive (Diagnostic)] # [diag (passes_missing_lang_item)] # [note] # [help] pub (crate) struct MissingLangItem { pub name : Symbol , }
/* FP:errors.rs-0185 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0093
/* FP:errors.rs-0186 */ # [derive (Diagnostic)] # [diag (passes_lang_item_fn_with_track_caller)] pub (crate) struct LangItemWithTrackCaller { # [primary_span] pub attr_span : Span , pub name : Symbol , # [label] pub sig_span : Span , }
/* FP:errors.rs-0187 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0094
/* FP:errors.rs-0188 */ # [derive (Diagnostic)] # [diag (passes_lang_item_fn_with_target_feature)] pub (crate) struct LangItemWithTargetFeature { # [primary_span] pub attr_span : Span , pub name : Symbol , # [label] pub sig_span : Span , }
/* FP:errors.rs-0189 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0095
/* FP:errors.rs-0190 */ # [derive (Diagnostic)] # [diag (passes_lang_item_on_incorrect_target , code = E0718)] pub (crate) struct LangItemOnIncorrectTarget { # [primary_span] # [label] pub span : Span , pub name : Symbol , pub expected_target : Target , pub actual_target : Target , }
/* FP:errors.rs-0191 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0096
/* FP:errors.rs-0192 */ # [derive (Diagnostic)] # [diag (passes_unknown_lang_item , code = E0522)] pub (crate) struct UnknownLangItem { # [primary_span] # [label] pub span : Span , pub name : Symbol , }
/* FP:errors.rs-0193 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0097
/* FP:errors.rs-0194 */ pub (crate) struct InvalidAttrAtCrateLevel { pub span : Span , pub sugg_span : Option < Span > , pub name : Symbol , pub item : Option < ItemFollowingInnerAttr > , }
/* FP:errors.rs-0195 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0098
/* FP:errors.rs-0196 */ # [derive (Clone , Copy)] pub (crate) struct ItemFollowingInnerAttr { pub span : Span , pub kind : & 'static str , }
/* FP:errors.rs-0197 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_IMPL_0099
/* FP:errors.rs-0198 */ impl < G : EmissionGuarantee > Diagnostic < '_ , G > for InvalidAttrAtCrateLevel { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { let mut diag = Diag :: new (dcx , level , fluent :: passes_invalid_attr_at_crate_level) ; diag . span (self . span) ; diag . arg ("name" , self . name) ; if let Some (span) = self . sugg_span { diag . span_suggestion_verbose (span , fluent :: passes_suggestion , String :: new () , Applicability :: MachineApplicable ,) ; } if let Some (item) = self . item { diag . arg ("kind" , item . kind) ; diag . span_label (item . span , fluent :: passes_invalid_attr_at_crate_level_item) ; } diag } }
/* FP:errors.rs-0199 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0100
/* FP:errors.rs-0200 */ # [derive (Diagnostic)] # [diag (passes_duplicate_diagnostic_item_in_crate)] pub (crate) struct DuplicateDiagnosticItemInCrate { # [primary_span] pub duplicate_span : Option < Span > , # [note (passes_diagnostic_item_first_defined)] pub orig_span : Option < Span > , # [note] pub different_crates : bool , pub crate_name : Symbol , pub orig_crate_name : Symbol , pub name : Symbol , }
/* FP:errors.rs-0201 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0101
/* FP:errors.rs-0202 */ # [derive (Diagnostic)] # [diag (passes_layout_abi)] pub (crate) struct LayoutAbi { # [primary_span] pub span : Span , pub abi : String , }
/* FP:errors.rs-0203 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0102
/* FP:errors.rs-0204 */ # [derive (Diagnostic)] # [diag (passes_layout_align)] pub (crate) struct LayoutAlign { # [primary_span] pub span : Span , pub align : String , }
/* FP:errors.rs-0205 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0103
/* FP:errors.rs-0206 */ # [derive (Diagnostic)] # [diag (passes_layout_size)] pub (crate) struct LayoutSize { # [primary_span] pub span : Span , pub size : String , }
/* FP:errors.rs-0207 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0104
/* FP:errors.rs-0208 */ # [derive (Diagnostic)] # [diag (passes_layout_homogeneous_aggregate)] pub (crate) struct LayoutHomogeneousAggregate { # [primary_span] pub span : Span , pub homogeneous_aggregate : String , }
/* FP:errors.rs-0209 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0105
/* FP:errors.rs-0210 */ # [derive (Diagnostic)] # [diag (passes_layout_of)] pub (crate) struct LayoutOf < 'tcx > { # [primary_span] pub span : Span , pub normalized_ty : Ty < 'tcx > , pub ty_layout : String , }
/* FP:errors.rs-0211 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0106
/* FP:errors.rs-0212 */ # [derive (Diagnostic)] # [diag (passes_layout_invalid_attribute)] pub (crate) struct LayoutInvalidAttribute { # [primary_span] pub span : Span , }
/* FP:errors.rs-0213 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0107
/* FP:errors.rs-0214 */ # [derive (Diagnostic)] # [diag (passes_abi_of)] pub (crate) struct AbiOf { # [primary_span] pub span : Span , pub fn_name : Symbol , pub fn_abi : String , }
/* FP:errors.rs-0215 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0108
/* FP:errors.rs-0216 */ # [derive (Diagnostic)] # [diag (passes_abi_ne)] pub (crate) struct AbiNe { # [primary_span] pub span : Span , pub left : String , pub right : String , }
/* FP:errors.rs-0217 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0109
/* FP:errors.rs-0218 */ # [derive (Diagnostic)] # [diag (passes_abi_invalid_attribute)] pub (crate) struct AbiInvalidAttribute { # [primary_span] pub span : Span , }
/* FP:errors.rs-0219 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0110
/* FP:errors.rs-0220 */ # [derive (Diagnostic)] # [diag (passes_unrecognized_argument)] pub (crate) struct UnrecognizedArgument { # [primary_span] pub span : Span , }
/* FP:errors.rs-0221 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0111
/* FP:errors.rs-0222 */ # [derive (Diagnostic)] # [diag (passes_feature_stable_twice , code = E0711)] pub (crate) struct FeatureStableTwice { # [primary_span] pub span : Span , pub feature : Symbol , pub since : Symbol , pub prev_since : Symbol , }
/* FP:errors.rs-0223 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0112
/* FP:errors.rs-0224 */ # [derive (Diagnostic)] # [diag (passes_feature_previously_declared , code = E0711)] pub (crate) struct FeaturePreviouslyDeclared < 'a > { # [primary_span] pub span : Span , pub feature : Symbol , pub declared : & 'a str , pub prev_declared : & 'a str , }
/* FP:errors.rs-0225 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0113
/* FP:errors.rs-0226 */ # [derive (Diagnostic)] # [diag (passes_attr_only_in_functions)] pub (crate) struct AttrOnlyInFunctions { # [primary_span] pub span : Span , pub attr : Symbol , }
/* FP:errors.rs-0227 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0114
/* FP:errors.rs-0228 */ # [derive (Diagnostic)] # [diag (passes_multiple_rustc_main , code = E0137)] pub (crate) struct MultipleRustcMain { # [primary_span] pub span : Span , # [label (passes_first)] pub first : Span , # [label (passes_additional)] pub additional : Span , }
/* FP:errors.rs-0229 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0115
/* FP:errors.rs-0230 */ # [derive (Diagnostic)] # [diag (passes_extern_main)] pub (crate) struct ExternMain { # [primary_span] pub span : Span , }
/* FP:errors.rs-0231 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0116
/* FP:errors.rs-0232 */ pub (crate) struct NoMainErr { pub sp : Span , pub crate_name : Symbol , pub has_filename : bool , pub filename : PathBuf , pub file_empty : bool , pub non_main_fns : Vec < Span > , pub main_def_opt : Option < MainDefinition > , pub add_teach_note : bool , }
/* FP:errors.rs-0233 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_IMPL_0117
/* FP:errors.rs-0234 */ impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for NoMainErr { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let mut diag = Diag :: new (dcx , level , fluent :: passes_no_main_function) ; diag . span (DUMMY_SP) ; diag . code (E0601) ; diag . arg ("crate_name" , self . crate_name) ; diag . arg ("filename" , self . filename) ; diag . arg ("has_filename" , self . has_filename) ; let note = if ! self . non_main_fns . is_empty () { for & span in & self . non_main_fns { diag . span_note (span , fluent :: passes_here_is_main) ; } diag . note (fluent :: passes_one_or_more_possible_main) ; diag . help (fluent :: passes_consider_moving_main) ; fluent :: passes_main_must_be_defined_at_crate } else if self . has_filename { fluent :: passes_consider_adding_main_to_file } else { fluent :: passes_consider_adding_main_at_crate } ; if self . file_empty { diag . note (note) ; } else { diag . span (self . sp . shrink_to_hi ()) ; diag . span_label (self . sp . shrink_to_hi () , note) ; } if let Some (main_def) = self . main_def_opt && main_def . opt_fn_def_id () . is_none () { diag . span_label (main_def . span , fluent :: passes_non_function_main) ; } if self . add_teach_note { diag . note (fluent :: passes_teach_note) ; } diag } }
/* FP:errors.rs-0235 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0118
/* FP:errors.rs-0236 */ pub (crate) struct DuplicateLangItem { pub local_span : Option < Span > , pub lang_item_name : Symbol , pub crate_name : Symbol , pub dependency_of : Option < Symbol > , pub is_local : bool , pub path : String , pub first_defined_span : Option < Span > , pub orig_crate_name : Option < Symbol > , pub orig_dependency_of : Option < Symbol > , pub orig_is_local : bool , pub orig_path : String , pub (crate) duplicate : Duplicate , }
/* FP:errors.rs-0237 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_IMPL_0119
/* FP:errors.rs-0238 */ impl < G : EmissionGuarantee > Diagnostic < '_ , G > for DuplicateLangItem { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { let mut diag = Diag :: new (dcx , level , match self . duplicate { Duplicate :: Plain => fluent :: passes_duplicate_lang_item , Duplicate :: Crate => fluent :: passes_duplicate_lang_item_crate , Duplicate :: CrateDepends => fluent :: passes_duplicate_lang_item_crate_depends , } ,) ; diag . code (E0152) ; diag . arg ("lang_item_name" , self . lang_item_name) ; diag . arg ("crate_name" , self . crate_name) ; if let Some (dependency_of) = self . dependency_of { diag . arg ("dependency_of" , dependency_of) ; } diag . arg ("path" , self . path) ; if let Some (orig_crate_name) = self . orig_crate_name { diag . arg ("orig_crate_name" , orig_crate_name) ; } if let Some (orig_dependency_of) = self . orig_dependency_of { diag . arg ("orig_dependency_of" , orig_dependency_of) ; } diag . arg ("orig_path" , self . orig_path) ; if let Some (span) = self . local_span { diag . span (span) ; } if let Some (span) = self . first_defined_span { diag . span_note (span , fluent :: passes_first_defined_span) ; } else { if self . orig_dependency_of . is_none () { diag . note (fluent :: passes_first_defined_crate) ; } else { diag . note (fluent :: passes_first_defined_crate_depends) ; } if self . orig_is_local { diag . note (fluent :: passes_first_definition_local) ; } else { diag . note (fluent :: passes_first_definition_path) ; } if self . is_local { diag . note (fluent :: passes_second_definition_local) ; } else { diag . note (fluent :: passes_second_definition_path) ; } } diag } }
/* FP:errors.rs-0239 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0120
/* FP:errors.rs-0240 */ # [derive (Diagnostic)] # [diag (passes_incorrect_target , code = E0718)] pub (crate) struct IncorrectTarget < 'a > { # [primary_span] pub span : Span , # [label] pub generics_span : Span , pub name : & 'a str , pub kind : & 'static str , pub num : usize , pub actual_num : usize , pub at_least : bool , }
/* FP:errors.rs-0241 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0121
/* FP:errors.rs-0242 */ # [derive (Diagnostic)] # [diag (passes_incorrect_crate_type)] pub (crate) struct IncorrectCrateType { # [primary_span] pub span : Span , }
/* FP:errors.rs-0243 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0122
/* FP:errors.rs-0244 */ # [derive (LintDiagnostic)] # [diag (passes_useless_assignment)] pub (crate) struct UselessAssignment < 'a > { pub is_field_assign : bool , pub ty : Ty < 'a > , }
/* FP:errors.rs-0245 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0123
/* FP:errors.rs-0246 */ # [derive (LintDiagnostic)] # [diag (passes_inline_ignored_for_exported)] # [help] pub (crate) struct InlineIgnoredForExported { }
/* FP:errors.rs-0247 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0124
/* FP:errors.rs-0248 */ # [derive (Diagnostic)] # [diag (passes_object_lifetime_err)] pub (crate) struct ObjectLifetimeErr { # [primary_span] pub span : Span , pub repr : String , }
/* FP:errors.rs-0249 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_ENUM_0125
/* FP:errors.rs-0250 */ # [derive (Diagnostic)] pub (crate) enum AttrApplication { # [diag (passes_attr_application_enum , code = E0517)] Enum { # [primary_span] hint_span : Span , # [label] span : Span , } , # [diag (passes_attr_application_struct , code = E0517)] Struct { # [primary_span] hint_span : Span , # [label] span : Span , } , # [diag (passes_attr_application_struct_union , code = E0517)] StructUnion { # [primary_span] hint_span : Span , # [label] span : Span , } , # [diag (passes_attr_application_struct_enum_union , code = E0517)] StructEnumUnion { # [primary_span] hint_span : Span , # [label] span : Span , } , }
/* FP:errors.rs-0251 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0126
/* FP:errors.rs-0252 */ # [derive (Diagnostic)] # [diag (passes_transparent_incompatible , code = E0692)] pub (crate) struct TransparentIncompatible { # [primary_span] pub hint_spans : Vec < Span > , pub target : String , }
/* FP:errors.rs-0253 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0127
/* FP:errors.rs-0254 */ # [derive (Diagnostic)] # [diag (passes_deprecated_attribute , code = E0549)] pub (crate) struct DeprecatedAttribute { # [primary_span] pub span : Span , }
/* FP:errors.rs-0255 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0128
/* FP:errors.rs-0256 */ # [derive (Diagnostic)] # [diag (passes_useless_stability)] pub (crate) struct UselessStability { # [primary_span] # [label] pub span : Span , # [label (passes_item)] pub item_sp : Span , }
/* FP:errors.rs-0257 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0129
/* FP:errors.rs-0258 */ # [derive (Diagnostic)] # [diag (passes_cannot_stabilize_deprecated)] pub (crate) struct CannotStabilizeDeprecated { # [primary_span] # [label] pub span : Span , # [label (passes_item)] pub item_sp : Span , }
/* FP:errors.rs-0259 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0130
/* FP:errors.rs-0260 */ # [derive (Diagnostic)] # [diag (passes_unstable_attr_for_already_stable_feature)] pub (crate) struct UnstableAttrForAlreadyStableFeature { # [primary_span] # [label] # [help] pub attr_span : Span , # [label (passes_item)] pub item_span : Span , }
/* FP:errors.rs-0261 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0131
/* FP:errors.rs-0262 */ # [derive (Diagnostic)] # [diag (passes_missing_stability_attr)] pub (crate) struct MissingStabilityAttr < 'a > { # [primary_span] pub span : Span , pub descr : & 'a str , }
/* FP:errors.rs-0263 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0132
/* FP:errors.rs-0264 */ # [derive (Diagnostic)] # [diag (passes_missing_const_stab_attr)] pub (crate) struct MissingConstStabAttr < 'a > { # [primary_span] pub span : Span , pub descr : & 'a str , }
/* FP:errors.rs-0265 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0133
/* FP:errors.rs-0266 */ # [derive (Diagnostic)] # [diag (passes_trait_impl_const_stable)] # [note] pub (crate) struct TraitImplConstStable { # [primary_span] pub span : Span , }
/* FP:errors.rs-0267 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0134
/* FP:errors.rs-0268 */ # [derive (Diagnostic)] # [diag (passes_trait_impl_const_stability_mismatch)] pub (crate) struct TraitImplConstStabilityMismatch { # [primary_span] pub span : Span , # [subdiagnostic] pub impl_stability : ImplConstStability , # [subdiagnostic] pub trait_stability : TraitConstStability , }
/* FP:errors.rs-0269 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_ENUM_0135
/* FP:errors.rs-0270 */ # [derive (Subdiagnostic)] pub (crate) enum TraitConstStability { # [note (passes_trait_impl_const_stability_mismatch_trait_stable)] Stable { # [primary_span] span : Span , } , # [note (passes_trait_impl_const_stability_mismatch_trait_unstable)] Unstable { # [primary_span] span : Span , } , }
/* FP:errors.rs-0271 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_ENUM_0136
/* FP:errors.rs-0272 */ # [derive (Subdiagnostic)] pub (crate) enum ImplConstStability { # [note (passes_trait_impl_const_stability_mismatch_impl_stable)] Stable { # [primary_span] span : Span , } , # [note (passes_trait_impl_const_stability_mismatch_impl_unstable)] Unstable { # [primary_span] span : Span , } , }
/* FP:errors.rs-0273 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0137
/* FP:errors.rs-0274 */ # [derive (Diagnostic)] # [diag (passes_unknown_feature , code = E0635)] pub (crate) struct UnknownFeature { # [primary_span] pub span : Span , pub feature : Symbol , }
/* FP:errors.rs-0275 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0138
/* FP:errors.rs-0276 */ # [derive (Diagnostic)] # [diag (passes_unknown_feature_alias , code = E0635)] pub (crate) struct RenamedFeature { # [primary_span] pub span : Span , pub feature : Symbol , pub alias : Symbol , }
/* FP:errors.rs-0277 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0139
/* FP:errors.rs-0278 */ # [derive (Diagnostic)] # [diag (passes_implied_feature_not_exist)] pub (crate) struct ImpliedFeatureNotExist { # [primary_span] pub span : Span , pub feature : Symbol , pub implied_by : Symbol , }
/* FP:errors.rs-0279 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0140
/* FP:errors.rs-0280 */ # [derive (Diagnostic)] # [diag (passes_duplicate_feature_err , code = E0636)] pub (crate) struct DuplicateFeatureErr { # [primary_span] pub span : Span , pub feature : Symbol , }
/* FP:errors.rs-0281 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0141
/* FP:errors.rs-0282 */ # [derive (Diagnostic)] # [diag (passes_missing_const_err)] pub (crate) struct MissingConstErr { # [primary_span] # [help] pub fn_sig_span : Span , }
/* FP:errors.rs-0283 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0142
/* FP:errors.rs-0284 */ # [derive (Diagnostic)] # [diag (passes_const_stable_not_stable)] pub (crate) struct ConstStableNotStable { # [primary_span] pub fn_sig_span : Span , # [label] pub const_span : Span , }
/* FP:errors.rs-0285 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_ENUM_0143
/* FP:errors.rs-0286 */ # [derive (LintDiagnostic)] pub (crate) enum MultipleDeadCodes < 'tcx > { # [diag (passes_dead_codes)] DeadCodes { multiple : bool , num : usize , descr : & 'tcx str , participle : & 'tcx str , name_list : DiagSymbolList , # [subdiagnostic] enum_variants_with_same_name : Vec < EnumVariantSameName < 'tcx > > , # [subdiagnostic] parent_info : Option < ParentInfo < 'tcx > > , # [subdiagnostic] ignored_derived_impls : Option < IgnoredDerivedImpls > , } , # [diag (passes_dead_codes)] UnusedTupleStructFields { multiple : bool , num : usize , descr : & 'tcx str , participle : & 'tcx str , name_list : DiagSymbolList , # [subdiagnostic] change_fields_suggestion : ChangeFields , # [subdiagnostic] parent_info : Option < ParentInfo < 'tcx > > , # [subdiagnostic] ignored_derived_impls : Option < IgnoredDerivedImpls > , } , }
/* FP:errors.rs-0287 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0144
/* FP:errors.rs-0288 */ # [derive (Subdiagnostic)] # [note (passes_enum_variant_same_name)] pub (crate) struct EnumVariantSameName < 'tcx > { # [primary_span] pub variant_span : Span , pub dead_name : Symbol , pub dead_descr : & 'tcx str , }
/* FP:errors.rs-0289 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0145
/* FP:errors.rs-0290 */ # [derive (Subdiagnostic)] # [label (passes_parent_info)] pub (crate) struct ParentInfo < 'tcx > { pub num : usize , pub descr : & 'tcx str , pub parent_descr : & 'tcx str , # [primary_span] pub span : Span , }
/* FP:errors.rs-0291 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0146
/* FP:errors.rs-0292 */ # [derive (Subdiagnostic)] # [note (passes_ignored_derived_impls)] pub (crate) struct IgnoredDerivedImpls { pub name : Symbol , pub trait_list : DiagSymbolList , pub trait_list_len : usize , }
/* FP:errors.rs-0293 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_ENUM_0147
/* FP:errors.rs-0294 */ # [derive (Subdiagnostic)] pub (crate) enum ChangeFields { # [multipart_suggestion (passes_change_fields_to_be_of_unit_type , applicability = "has-placeholders")] ChangeToUnitTypeOrRemove { num : usize , # [suggestion_part (code = "()")] spans : Vec < Span > , } , # [help (passes_remove_fields)] Remove { num : usize } , }
/* FP:errors.rs-0295 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0148
/* FP:errors.rs-0296 */ # [derive (Diagnostic)] # [diag (passes_proc_macro_bad_sig)] pub (crate) struct ProcMacroBadSig { # [primary_span] pub span : Span , pub kind : ProcMacroKind , }
/* FP:errors.rs-0297 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0149
/* FP:errors.rs-0298 */ # [derive (LintDiagnostic)] # [diag (passes_unreachable_due_to_uninhabited)] pub (crate) struct UnreachableDueToUninhabited < 'desc , 'tcx > { pub descr : & 'desc str , # [label] pub expr : Span , # [label (passes_label_orig)] # [note] pub orig : Span , pub ty : Ty < 'tcx > , }
/* FP:errors.rs-0299 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0150
/* FP:errors.rs-0300 */ # [derive (LintDiagnostic)] # [diag (passes_unused_var_maybe_capture_ref)] # [help] pub (crate) struct UnusedVarMaybeCaptureRef { pub name : String , }
/* FP:errors.rs-0301 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0151
/* FP:errors.rs-0302 */ # [derive (LintDiagnostic)] # [diag (passes_unused_capture_maybe_capture_ref)] # [help] pub (crate) struct UnusedCaptureMaybeCaptureRef { pub name : String , }
/* FP:errors.rs-0303 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0152
/* FP:errors.rs-0304 */ # [derive (LintDiagnostic)] # [diag (passes_unused_var_remove_field)] pub (crate) struct UnusedVarRemoveField { pub name : String , # [subdiagnostic] pub sugg : UnusedVarRemoveFieldSugg , }
/* FP:errors.rs-0305 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0153
/* FP:errors.rs-0306 */ # [derive (Subdiagnostic)] # [multipart_suggestion (passes_unused_var_remove_field_suggestion , applicability = "machine-applicable")] pub (crate) struct UnusedVarRemoveFieldSugg { # [suggestion_part (code = "")] pub spans : Vec < Span > , }
/* FP:errors.rs-0307 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0154
/* FP:errors.rs-0308 */ # [derive (LintDiagnostic)] # [diag (passes_unused_var_assigned_only)] # [note] pub (crate) struct UnusedVarAssignedOnly { pub name : String , # [subdiagnostic] pub typo : Option < PatternTypo > , }
/* FP:errors.rs-0309 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0155
/* FP:errors.rs-0310 */ # [derive (Subdiagnostic)] # [multipart_suggestion (passes_unused_var_typo , style = "verbose" , applicability = "machine-applicable")] pub (crate) struct PatternTypo { # [suggestion_part (code = "{code}")] pub span : Span , pub code : String , pub item_name : String , pub kind : String , }
/* FP:errors.rs-0311 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0156
/* FP:errors.rs-0312 */ # [derive (LintDiagnostic)] # [diag (passes_unnecessary_stable_feature)] pub (crate) struct UnnecessaryStableFeature { pub feature : Symbol , pub since : Symbol , }
/* FP:errors.rs-0313 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0157
/* FP:errors.rs-0314 */ # [derive (LintDiagnostic)] # [diag (passes_unnecessary_partial_stable_feature)] pub (crate) struct UnnecessaryPartialStableFeature { # [suggestion (code = "{implies}" , applicability = "maybe-incorrect")] pub span : Span , # [suggestion (passes_suggestion_remove , code = "" , applicability = "maybe-incorrect")] pub line : Span , pub feature : Symbol , pub since : Symbol , pub implies : Symbol , }
/* FP:errors.rs-0315 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0158
/* FP:errors.rs-0316 */ # [derive (LintDiagnostic)] # [diag (passes_ineffective_unstable_impl)] # [note] pub (crate) struct IneffectiveUnstableImpl ;
/* FP:errors.rs-0317 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0159
/* FP:errors.rs-0318 */ # [derive (LintDiagnostic)] # [diag (passes_unused_assign)] pub (crate) struct UnusedAssign { pub name : String , # [subdiagnostic] pub suggestion : Option < UnusedAssignSuggestion > , # [help] pub help : bool , }
/* FP:errors.rs-0319 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0160
/* FP:errors.rs-0320 */ # [derive (Subdiagnostic)] # [multipart_suggestion (passes_unused_assign_suggestion , applicability = "maybe-incorrect")] pub (crate) struct UnusedAssignSuggestion { pub pre : & 'static str , # [suggestion_part (code = "{pre}mut ")] pub ty_span : Option < Span > , # [suggestion_part (code = "")] pub ty_ref_span : Span , # [suggestion_part (code = "*")] pub ident_span : Span , # [suggestion_part (code = "")] pub expr_ref_span : Span , }
/* FP:errors.rs-0321 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0161
/* FP:errors.rs-0322 */ # [derive (LintDiagnostic)] # [diag (passes_unused_assign_passed)] # [help] pub (crate) struct UnusedAssignPassed { pub name : String , }
/* FP:errors.rs-0323 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0162
/* FP:errors.rs-0324 */ # [derive (LintDiagnostic)] # [diag (passes_unused_variable_try_prefix)] pub (crate) struct UnusedVariableTryPrefix { # [label] pub label : Option < Span > , # [subdiagnostic] pub string_interp : Vec < UnusedVariableStringInterp > , # [subdiagnostic] pub sugg : UnusedVariableSugg , pub name : String , # [subdiagnostic] pub typo : Option < PatternTypo > , }
/* FP:errors.rs-0325 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_ENUM_0163
/* FP:errors.rs-0326 */ # [derive (Subdiagnostic)] pub (crate) enum UnusedVariableSugg { # [multipart_suggestion (passes_suggestion , applicability = "maybe-incorrect")] TryPrefixSugg { # [suggestion_part (code = "_{name}")] spans : Vec < Span > , name : String , } , # [help (passes_unused_variable_args_in_macro)] NoSugg { # [primary_span] span : Span , name : String , } , }
/* FP:errors.rs-0327 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0164
/* FP:errors.rs-0328 */ pub (crate) struct UnusedVariableStringInterp { pub lit : Span , pub lo : Span , pub hi : Span , }
/* FP:errors.rs-0329 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_IMPL_0165
/* FP:errors.rs-0330 */ impl Subdiagnostic for UnusedVariableStringInterp { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . span_label (self . lit , crate :: fluent_generated :: passes_maybe_string_interpolation) ; diag . multipart_suggestion (crate :: fluent_generated :: passes_string_interpolation_only_works , vec ! [(self . lo , String :: from ("format!(")) , (self . hi , String :: from (")"))] , Applicability :: MachineApplicable ,) ; } }
/* FP:errors.rs-0331 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0166
/* FP:errors.rs-0332 */ # [derive (LintDiagnostic)] # [diag (passes_unused_variable_try_ignore)] pub (crate) struct UnusedVarTryIgnore { pub name : String , # [subdiagnostic] pub sugg : UnusedVarTryIgnoreSugg , }
/* FP:errors.rs-0333 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0167
/* FP:errors.rs-0334 */ # [derive (Subdiagnostic)] # [multipart_suggestion (passes_suggestion , applicability = "maybe-incorrect")] pub (crate) struct UnusedVarTryIgnoreSugg { # [suggestion_part (code = "{name}: _")] pub shorthands : Vec < Span > , # [suggestion_part (code = "_")] pub non_shorthands : Vec < Span > , pub name : String , }
/* FP:errors.rs-0335 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0168
/* FP:errors.rs-0336 */ # [derive (LintDiagnostic)] # [diag (passes_attr_crate_level)] # [note] pub (crate) struct AttrCrateLevelOnly { # [subdiagnostic] pub sugg : Option < AttrCrateLevelOnlySugg > , }
/* FP:errors.rs-0337 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0169
/* FP:errors.rs-0338 */ # [derive (Subdiagnostic)] # [suggestion (passes_suggestion , applicability = "maybe-incorrect" , code = "!" , style = "verbose")] pub (crate) struct AttrCrateLevelOnlySugg { # [primary_span] pub attr : Span , }
/* FP:errors.rs-0339 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0170
/* FP:errors.rs-0340 */ # [doc = " \"sanitize attribute not allowed here\""] # [derive (Diagnostic)] # [diag (passes_sanitize_attribute_not_allowed)] pub (crate) struct SanitizeAttributeNotAllowed { # [primary_span] pub attr_span : Span , # [doc = " \"not a function, impl block, or module\""] # [label (passes_not_fn_impl_mod)] pub not_fn_impl_mod : Option < Span > , # [doc = " \"function has no body\""] # [label (passes_no_body)] pub no_body : Option < Span > , # [doc = " \"sanitize attribute can be applied to a function (with body), impl block, or module\""] # [help] pub help : () , }
/* FP:errors.rs-0341 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0171
/* FP:errors.rs-0342 */ # [derive (Diagnostic)] # [diag (passes_rustc_const_stable_indirect_pairing)] pub (crate) struct RustcConstStableIndirectPairing { # [primary_span] pub span : Span , }
/* FP:errors.rs-0343 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0172
/* FP:errors.rs-0344 */ # [derive (Diagnostic)] # [diag (passes_unsupported_attributes_in_where)] # [help] pub (crate) struct UnsupportedAttributesInWhere { # [primary_span] pub span : MultiSpan , }
/* FP:errors.rs-0345 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_ENUM_0173
/* FP:errors.rs-0346 */ # [derive (Diagnostic)] pub (crate) enum UnexportableItem < 'a > { # [diag (passes_unexportable_item)] Item { # [primary_span] span : Span , descr : & 'a str , } , # [diag (passes_unexportable_generic_fn)] GenericFn (# [primary_span] Span) , # [diag (passes_unexportable_fn_abi)] FnAbi (# [primary_span] Span) , # [diag (passes_unexportable_type_repr)] TypeRepr (# [primary_span] Span) , # [diag (passes_unexportable_type_in_interface)] TypeInInterface { # [primary_span] span : Span , desc : & 'a str , ty : & 'a str , # [label] ty_span : Span , } , # [diag (passes_unexportable_priv_item)] PrivItem { # [primary_span] span : Span , # [note] vis_note : Span , vis_descr : & 'a str , } , # [diag (passes_unexportable_adt_with_private_fields)] AdtWithPrivFields { # [primary_span] span : Span , # [note] vis_note : Span , field_name : & 'a str , } , }
/* FP:errors.rs-0347 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0174
/* FP:errors.rs-0348 */ # [derive (Diagnostic)] # [diag (passes_repr_align_should_be_align)] pub (crate) struct ReprAlignShouldBeAlign { # [primary_span] # [help] pub span : Span , pub item : & 'static str , }
/* FP:errors.rs-0349 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0175
/* FP:errors.rs-0350 */ # [derive (Diagnostic)] # [diag (passes_repr_align_should_be_align_static)] pub (crate) struct ReprAlignShouldBeAlignStatic { # [primary_span] # [help] pub span : Span , pub item : & 'static str , }
/* FP:errors.rs-0351 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0176
/* FP:errors.rs-0352 */ # [derive (Diagnostic)] # [diag (passes_custom_mir_phase_requires_dialect)] pub (crate) struct CustomMirPhaseRequiresDialect { # [primary_span] pub attr_span : Span , # [label] pub phase_span : Span , }
/* FP:errors.rs-0353 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_errors_STRUCT_0177
/* FP:errors.rs-0354 */ # [derive (Diagnostic)] # [diag (passes_custom_mir_incompatible_dialect_and_phase)] pub (crate) struct CustomMirIncompatibleDialectAndPhase { pub dialect : MirDialect , pub phase : MirPhase , # [primary_span] pub attr_span : Span , # [label] pub dialect_span : Span , # [label] pub phase_span : Span , }