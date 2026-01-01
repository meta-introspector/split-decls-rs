/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_USE_0001
/* FP:errors.rs-0002 */ use crate :: rustc_complete :: codes :: * ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_USE_0002
/* FP:errors.rs-0004 */ use crate :: rustc_complete :: { Diag , DiagCtxtHandle , Diagnostic , EmissionGuarantee , Level , MultiSpan , SingleLabelManySpans , Subdiagnostic , } ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_USE_0003
/* FP:errors.rs-0006 */ use rustc_macros :: { Diagnostic , Subdiagnostic } ;
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_USE_0004
/* FP:errors.rs-0008 */ use crate :: rustc_complete :: { Ident , Span , Symbol } ;
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0005
/* FP:errors.rs-0010 */ # [derive (Diagnostic)] # [diag (builtin_macros_requires_cfg_pattern)] pub (crate) struct RequiresCfgPattern { # [primary_span] # [label] pub (crate) span : Span , }
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0006
/* FP:errors.rs-0012 */ # [derive (Diagnostic)] # [diag (builtin_macros_expected_one_cfg_pattern)] pub (crate) struct OneCfgPattern { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0007
/* FP:errors.rs-0014 */ # [derive (Diagnostic)] # [diag (builtin_macros_alloc_error_must_be_fn)] pub (crate) struct AllocErrorMustBeFn { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0008
/* FP:errors.rs-0016 */ # [derive (Diagnostic)] # [diag (builtin_macros_assert_requires_boolean)] pub (crate) struct AssertRequiresBoolean { # [primary_span] # [label] pub (crate) span : Span , }
/* FP:errors.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0009
/* FP:errors.rs-0018 */ # [derive (Diagnostic)] # [diag (builtin_macros_assert_requires_expression)] pub (crate) struct AssertRequiresExpression { # [primary_span] pub (crate) span : Span , # [suggestion (code = "" , applicability = "maybe-incorrect")] pub (crate) token : Span , }
/* FP:errors.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0010
/* FP:errors.rs-0020 */ # [derive (Diagnostic)] # [diag (builtin_macros_assert_missing_comma)] pub (crate) struct AssertMissingComma { # [primary_span] pub (crate) span : Span , # [suggestion (code = ", " , applicability = "maybe-incorrect" , style = "short")] pub (crate) comma : Span , }
/* FP:errors.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_ENUM_0011
/* FP:errors.rs-0022 */ # [derive (Diagnostic)] pub (crate) enum CfgAccessibleInvalid { # [diag (builtin_macros_cfg_accessible_unspecified_path)] UnspecifiedPath (# [primary_span] Span) , # [diag (builtin_macros_cfg_accessible_multiple_paths)] MultiplePaths (# [primary_span] Span) , # [diag (builtin_macros_cfg_accessible_literal_path)] LiteralPath (# [primary_span] Span) , # [diag (builtin_macros_cfg_accessible_has_args)] HasArguments (# [primary_span] Span) , }
/* FP:errors.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0012
/* FP:errors.rs-0024 */ # [derive (Diagnostic)] # [diag (builtin_macros_cfg_accessible_indeterminate)] pub (crate) struct CfgAccessibleIndeterminate { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0013
/* FP:errors.rs-0026 */ # [derive (Diagnostic)] # [diag (builtin_macros_concat_missing_literal)] # [note] pub (crate) struct ConcatMissingLiteral { # [primary_span] pub (crate) spans : Vec < Span > , }
/* FP:errors.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0014
/* FP:errors.rs-0028 */ # [derive (Diagnostic)] # [diag (builtin_macros_concat_bytestr)] pub (crate) struct ConcatBytestr { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0015
/* FP:errors.rs-0030 */ # [derive (Diagnostic)] # [diag (builtin_macros_concat_c_str_lit)] pub (crate) struct ConcatCStrLit { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0016
/* FP:errors.rs-0032 */ # [derive (Diagnostic)] # [diag (builtin_macros_export_macro_rules)] pub (crate) struct ExportMacroRules { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0017
/* FP:errors.rs-0034 */ # [derive (Diagnostic)] # [diag (builtin_macros_proc_macro)] pub (crate) struct ProcMacro { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0018
/* FP:errors.rs-0036 */ # [derive (Diagnostic)] # [diag (builtin_macros_trace_macros)] pub (crate) struct TraceMacros { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0019
/* FP:errors.rs-0038 */ # [derive (Diagnostic)] # [diag (builtin_macros_bench_sig)] pub (crate) struct BenchSig { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0020
/* FP:errors.rs-0040 */ # [derive (Diagnostic)] # [diag (builtin_macros_alloc_must_statics)] pub (crate) struct AllocMustStatics { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_USE_0021
/* FP:errors.rs-0042 */ pub (crate) use autodiff :: * ;
/* FP:errors.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_MOD_0022
/* FP:errors.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_USE_0023
/* FP:errors.rs-0046 */ pub (crate) use ad_fallback :: * ;
/* FP:errors.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_MOD_0024
/* FP:errors.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0025
/* FP:errors.rs-0050 */ # [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_invalid)] pub (crate) struct ConcatBytesInvalid { # [primary_span] pub (crate) span : Span , pub (crate) lit_kind : & 'static str , # [subdiagnostic] pub (crate) sugg : Option < ConcatBytesInvalidSuggestion > , # [note (builtin_macros_c_str_note)] pub (crate) cs_note : Option < () > , }
/* FP:errors.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_ENUM_0026
/* FP:errors.rs-0052 */ # [derive (Subdiagnostic)] pub (crate) enum ConcatBytesInvalidSuggestion { # [suggestion (builtin_macros_byte_char , code = "b{snippet}" , applicability = "machine-applicable")] CharLit { # [primary_span] span : Span , snippet : String , } , # [suggestion (builtin_macros_byte_str , code = "b{snippet}" , applicability = "machine-applicable")] StrLit { # [primary_span] span : Span , snippet : String , } , # [note (builtin_macros_c_str_note)] # [suggestion (builtin_macros_c_str , code = "{as_bstr}" , applicability = "machine-applicable")] CStrLit { # [primary_span] span : Span , as_bstr : String , } , # [suggestion (builtin_macros_number_array , code = "[{snippet}]" , applicability = "machine-applicable")] IntLit { # [primary_span] span : Span , snippet : String , } , }
/* FP:errors.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0027
/* FP:errors.rs-0054 */ # [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_oob)] pub (crate) struct ConcatBytesOob { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0028
/* FP:errors.rs-0056 */ # [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_non_u8)] pub (crate) struct ConcatBytesNonU8 { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0029
/* FP:errors.rs-0058 */ # [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_missing_literal)] # [note] pub (crate) struct ConcatBytesMissingLiteral { # [primary_span] pub (crate) spans : Vec < Span > , }
/* FP:errors.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0030
/* FP:errors.rs-0060 */ # [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_array)] pub (crate) struct ConcatBytesArray { # [primary_span] pub (crate) span : Span , # [note] # [help] pub (crate) bytestr : bool , }
/* FP:errors.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0031
/* FP:errors.rs-0062 */ # [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_bad_repeat)] pub (crate) struct ConcatBytesBadRepeat { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0032
/* FP:errors.rs-0064 */ # [derive (Diagnostic)] # [diag (builtin_macros_bad_derive_target , code = E0774)] pub (crate) struct BadDeriveTarget { # [primary_span] # [label] pub (crate) span : Span , # [label (builtin_macros_label2)] pub (crate) item : Span , }
/* FP:errors.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0033
/* FP:errors.rs-0066 */ # [derive (Diagnostic)] # [diag (builtin_macros_tests_not_support)] pub (crate) struct TestsNotSupport { }
/* FP:errors.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0034
/* FP:errors.rs-0068 */ # [derive (Diagnostic)] # [diag (builtin_macros_unexpected_lit , code = E0777)] pub (crate) struct BadDeriveLit { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub help : BadDeriveLitHelp , }
/* FP:errors.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_ENUM_0035
/* FP:errors.rs-0070 */ # [derive (Subdiagnostic)] pub (crate) enum BadDeriveLitHelp { # [help (builtin_macros_str_lit)] StrLit { sym : Symbol } , # [help (builtin_macros_other)] Other , }
/* FP:errors.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0036
/* FP:errors.rs-0072 */ # [derive (Diagnostic)] # [diag (builtin_macros_derive_path_args_list)] pub (crate) struct DerivePathArgsList { # [suggestion (code = "" , applicability = "machine-applicable")] # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0037
/* FP:errors.rs-0074 */ # [derive (Diagnostic)] # [diag (builtin_macros_derive_path_args_value)] pub (crate) struct DerivePathArgsValue { # [suggestion (code = "" , applicability = "machine-applicable")] # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0038
/* FP:errors.rs-0076 */ # [derive (Diagnostic)] # [diag (builtin_macros_no_default_variant , code = E0665)] pub (crate) struct NoDefaultVariant { # [primary_span] pub (crate) span : Span , # [label] pub (crate) item_span : Span , # [subdiagnostic] pub (crate) suggs : Vec < NoDefaultVariantSugg > , }
/* FP:errors.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0039
/* FP:errors.rs-0078 */ # [derive (Subdiagnostic)] # [suggestion (builtin_macros_suggestion , code = "#[default] " , applicability = "maybe-incorrect")] pub (crate) struct NoDefaultVariantSugg { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0040
/* FP:errors.rs-0080 */ # [derive (Diagnostic)] # [diag (builtin_macros_multiple_defaults)] # [note] pub (crate) struct MultipleDefaults { # [primary_span] pub (crate) span : Span , # [label] pub (crate) first : Span , # [label (builtin_macros_additional)] pub additional : Vec < Span > , # [subdiagnostic] pub suggs : Vec < MultipleDefaultsSugg > , }
/* FP:errors.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0041
/* FP:errors.rs-0082 */ # [derive (Subdiagnostic)] # [multipart_suggestion (builtin_macros_suggestion , applicability = "maybe-incorrect" , style = "tool-only")] pub (crate) struct MultipleDefaultsSugg { # [suggestion_part (code = "")] pub (crate) spans : Vec < Span > , pub (crate) ident : Ident , }
/* FP:errors.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0042
/* FP:errors.rs-0084 */ # [derive (Diagnostic)] # [diag (builtin_macros_non_unit_default)] # [help] pub (crate) struct NonUnitDefault { # [primary_span] pub (crate) span : Span , pub (crate) post : & 'static str , }
/* FP:errors.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0043
/* FP:errors.rs-0086 */ # [derive (Diagnostic)] # [diag (builtin_macros_non_exhaustive_default)] # [help] pub (crate) struct NonExhaustiveDefault { # [primary_span] pub (crate) span : Span , # [label] pub (crate) non_exhaustive : Span , }
/* FP:errors.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0044
/* FP:errors.rs-0088 */ # [derive (Diagnostic)] # [diag (builtin_macros_multiple_default_attrs)] # [note] pub (crate) struct MultipleDefaultAttrs { # [primary_span] pub (crate) span : Span , # [label] pub (crate) first : Span , # [label (builtin_macros_label_again)] pub (crate) first_rest : Span , # [help] pub (crate) rest : MultiSpan , pub (crate) only_one : bool , # [subdiagnostic] pub (crate) sugg : MultipleDefaultAttrsSugg , }
/* FP:errors.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0045
/* FP:errors.rs-0090 */ # [derive (Subdiagnostic)] # [multipart_suggestion (builtin_macros_help , applicability = "machine-applicable" , style = "tool-only")] pub (crate) struct MultipleDefaultAttrsSugg { # [suggestion_part (code = "")] pub (crate) spans : Vec < Span > , }
/* FP:errors.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0046
/* FP:errors.rs-0092 */ # [derive (Diagnostic)] # [diag (builtin_macros_default_arg)] pub (crate) struct DefaultHasArg { # [primary_span] # [suggestion (code = "#[default]" , style = "hidden" , applicability = "maybe-incorrect")] pub (crate) span : Span , }
/* FP:errors.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0047
/* FP:errors.rs-0094 */ # [derive (Diagnostic)] # [diag (builtin_macros_derive_from_wrong_target)] # [note (builtin_macros_derive_from_usage_note)] pub (crate) struct DeriveFromWrongTarget < 'a > { # [primary_span] pub (crate) span : MultiSpan , pub (crate) kind : & 'a str , }
/* FP:errors.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0048
/* FP:errors.rs-0096 */ # [derive (Diagnostic)] # [diag (builtin_macros_derive_from_wrong_field_count)] # [note (builtin_macros_derive_from_usage_note)] pub (crate) struct DeriveFromWrongFieldCount { # [primary_span] pub (crate) span : MultiSpan , pub (crate) multiple_fields : bool , }
/* FP:errors.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0049
/* FP:errors.rs-0098 */ # [derive (Diagnostic)] # [diag (builtin_macros_derive_macro_call)] pub (crate) struct DeriveMacroCall { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0050
/* FP:errors.rs-0100 */ # [derive (Diagnostic)] # [diag (builtin_macros_cannot_derive_union)] pub (crate) struct DeriveUnion { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0051
/* FP:errors.rs-0102 */ # [derive (Diagnostic)] # [diag (builtin_macros_env_takes_args)] pub (crate) struct EnvTakesArgs { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0052
/* FP:errors.rs-0104 */ pub (crate) struct EnvNotDefinedWithUserMessage { pub (crate) span : Span , pub (crate) msg_from_user : Symbol , }
/* FP:errors.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_IMPL_0053
/* FP:errors.rs-0106 */ impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for EnvNotDefinedWithUserMessage { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { # [expect (rustc :: untranslatable_diagnostic , reason = "cannot translate user-provided messages")] let mut diag = Diag :: new (dcx , level , self . msg_from_user . to_string ()) ; diag . span (self . span) ; diag } }
/* FP:errors.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_ENUM_0054
/* FP:errors.rs-0108 */ # [derive (Diagnostic)] pub (crate) enum EnvNotDefined < 'a > { # [diag (builtin_macros_env_not_defined)] # [help (builtin_macros_cargo)] CargoEnvVar { # [primary_span] span : Span , var : Symbol , var_expr : & 'a crate :: rustc_ast :: Expr , } , # [diag (builtin_macros_env_not_defined)] # [help (builtin_macros_custom)] CustomEnvVar { # [primary_span] span : Span , var : Symbol , var_expr : & 'a crate :: rustc_ast :: Expr , } , }
/* FP:errors.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0055
/* FP:errors.rs-0110 */ # [derive (Diagnostic)] # [diag (builtin_macros_env_not_unicode)] pub (crate) struct EnvNotUnicode { # [primary_span] pub (crate) span : Span , pub (crate) var : Symbol , }
/* FP:errors.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0056
/* FP:errors.rs-0112 */ # [derive (Diagnostic)] # [diag (builtin_macros_format_requires_string)] pub (crate) struct FormatRequiresString { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0057
/* FP:errors.rs-0114 */ # [derive (Diagnostic)] # [diag (builtin_macros_format_duplicate_arg)] pub (crate) struct FormatDuplicateArg { # [primary_span] pub (crate) span : Span , # [label (builtin_macros_label1)] pub (crate) prev : Span , # [label (builtin_macros_label2)] pub (crate) duplicate : Span , pub (crate) ident : Ident , }
/* FP:errors.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0058
/* FP:errors.rs-0116 */ # [derive (Diagnostic)] # [diag (builtin_macros_format_positional_after_named)] pub (crate) struct PositionalAfterNamed { # [primary_span] # [label] pub (crate) span : Span , # [label (builtin_macros_named_args)] pub (crate) args : Vec < Span > , }
/* FP:errors.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0059
/* FP:errors.rs-0118 */ # [derive (Diagnostic)] # [diag (builtin_macros_format_string_invalid)] pub (crate) struct InvalidFormatString { # [primary_span] # [label] pub (crate) span : Span , pub (crate) desc : String , pub (crate) label1 : String , # [subdiagnostic] pub (crate) note_ : Option < InvalidFormatStringNote > , # [subdiagnostic] pub (crate) label_ : Option < InvalidFormatStringLabel > , # [subdiagnostic] pub (crate) sugg_ : Option < InvalidFormatStringSuggestion > , }
/* FP:errors.rs-0119 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0060
/* FP:errors.rs-0120 */ # [derive (Subdiagnostic)] # [note (builtin_macros_note)] pub (crate) struct InvalidFormatStringNote { pub (crate) note : String , }
/* FP:errors.rs-0121 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0061
/* FP:errors.rs-0122 */ # [derive (Subdiagnostic)] # [label (builtin_macros_second_label)] pub (crate) struct InvalidFormatStringLabel { # [primary_span] pub (crate) span : Span , pub (crate) label : String , }
/* FP:errors.rs-0123 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_ENUM_0062
/* FP:errors.rs-0124 */ # [derive (Subdiagnostic)] pub (crate) enum InvalidFormatStringSuggestion { # [multipart_suggestion (builtin_macros_format_use_positional , style = "verbose" , applicability = "machine-applicable")] UsePositional { # [suggestion_part (code = "{len}")] captured : Span , len : String , # [suggestion_part (code = ", {arg}")] span : Span , arg : String , } , # [suggestion (builtin_macros_format_remove_raw_ident , code = "" , applicability = "machine-applicable")] RemoveRawIdent { # [primary_span] span : Span , } , # [suggestion (builtin_macros_format_reorder_format_parameter , code = "{replacement}" , style = "verbose" , applicability = "machine-applicable")] ReorderFormatParameter { # [primary_span] span : Span , replacement : String , } , }
/* FP:errors.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0063
/* FP:errors.rs-0126 */ # [derive (Diagnostic)] # [diag (builtin_macros_format_no_arg_named)] # [note] # [note (builtin_macros_note2)] pub (crate) struct FormatNoArgNamed { # [primary_span] pub (crate) span : Span , pub (crate) name : Symbol , }
/* FP:errors.rs-0127 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0064
/* FP:errors.rs-0128 */ # [derive (Diagnostic)] # [diag (builtin_macros_format_unknown_trait)] # [note] pub (crate) struct FormatUnknownTrait < 'a > { # [primary_span] pub (crate) span : Span , pub (crate) ty : & 'a str , # [subdiagnostic] pub (crate) suggs : Vec < FormatUnknownTraitSugg > , }
/* FP:errors.rs-0129 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0065
/* FP:errors.rs-0130 */ # [derive (Subdiagnostic)] # [suggestion (builtin_macros_suggestion , code = "{fmt}" , style = "tool-only" , applicability = "maybe-incorrect")] pub (crate) struct FormatUnknownTraitSugg { # [primary_span] pub span : Span , pub fmt : & 'static str , pub trait_name : & 'static str , }
/* FP:errors.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0066
/* FP:errors.rs-0132 */ # [derive (Diagnostic)] # [diag (builtin_macros_format_unused_arg)] pub (crate) struct FormatUnusedArg { # [primary_span] # [label (builtin_macros_format_unused_arg)] pub (crate) span : Span , pub (crate) named : bool , }
/* FP:errors.rs-0133 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_IMPL_0067
/* FP:errors.rs-0134 */ impl Subdiagnostic for FormatUnusedArg { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . arg ("named" , self . named) ; let msg = diag . eagerly_translate (crate :: fluent_generated :: builtin_macros_format_unused_arg) ; diag . remove_arg ("named") ; diag . span_label (self . span , msg) ; } }
/* FP:errors.rs-0135 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0068
/* FP:errors.rs-0136 */ # [derive (Diagnostic)] # [diag (builtin_macros_format_unused_args)] pub (crate) struct FormatUnusedArgs { # [primary_span] pub (crate) unused : Vec < Span > , # [label] pub (crate) fmt : Span , # [subdiagnostic] pub (crate) unused_labels : Vec < FormatUnusedArg > , }
/* FP:errors.rs-0137 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0069
/* FP:errors.rs-0138 */ # [derive (Diagnostic)] # [diag (builtin_macros_format_pos_mismatch)] pub (crate) struct FormatPositionalMismatch { # [primary_span] pub (crate) span : MultiSpan , pub (crate) n : usize , pub (crate) desc : String , # [subdiagnostic] pub (crate) highlight : SingleLabelManySpans , }
/* FP:errors.rs-0139 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0070
/* FP:errors.rs-0140 */ # [derive (Diagnostic)] # [diag (builtin_macros_format_redundant_args)] pub (crate) struct FormatRedundantArgs { # [primary_span] pub (crate) span : MultiSpan , pub (crate) n : usize , # [note] pub (crate) note : MultiSpan , # [subdiagnostic] pub (crate) sugg : Option < FormatRedundantArgsSugg > , }
/* FP:errors.rs-0141 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0071
/* FP:errors.rs-0142 */ # [derive (Subdiagnostic)] # [multipart_suggestion (builtin_macros_suggestion , applicability = "machine-applicable")] pub (crate) struct FormatRedundantArgsSugg { # [suggestion_part (code = "")] pub (crate) spans : Vec < Span > , }
/* FP:errors.rs-0143 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0072
/* FP:errors.rs-0144 */ # [derive (Diagnostic)] # [diag (builtin_macros_test_case_non_item)] pub (crate) struct TestCaseNonItem { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0145 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0073
/* FP:errors.rs-0146 */ # [derive (Diagnostic)] # [diag (builtin_macros_test_bad_fn)] pub (crate) struct TestBadFn { # [primary_span] pub (crate) span : Span , # [label] pub (crate) cause : Span , pub (crate) kind : & 'static str , }
/* FP:errors.rs-0147 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0074
/* FP:errors.rs-0148 */ # [derive (Diagnostic)] # [diag (builtin_macros_asm_explicit_register_name)] pub (crate) struct AsmExplicitRegisterName { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0149 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0075
/* FP:errors.rs-0150 */ # [derive (Diagnostic)] # [diag (builtin_macros_asm_mutually_exclusive)] pub (crate) struct AsmMutuallyExclusive { # [primary_span] pub (crate) spans : Vec < Span > , pub (crate) opt1 : & 'static str , pub (crate) opt2 : & 'static str , }
/* FP:errors.rs-0151 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0076
/* FP:errors.rs-0152 */ # [derive (Diagnostic)] # [diag (builtin_macros_asm_pure_combine)] pub (crate) struct AsmPureCombine { # [primary_span] pub (crate) spans : Vec < Span > , }
/* FP:errors.rs-0153 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0077
/* FP:errors.rs-0154 */ # [derive (Diagnostic)] # [diag (builtin_macros_asm_pure_no_output)] pub (crate) struct AsmPureNoOutput { # [primary_span] pub (crate) spans : Vec < Span > , }
/* FP:errors.rs-0155 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0078
/* FP:errors.rs-0156 */ # [derive (Diagnostic)] # [diag (builtin_macros_asm_modifier_invalid)] pub (crate) struct AsmModifierInvalid { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0157 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0079
/* FP:errors.rs-0158 */ # [derive (Diagnostic)] # [diag (builtin_macros_asm_attribute_not_supported)] pub (crate) struct AsmAttributeNotSupported { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0159 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0080
/* FP:errors.rs-0160 */ # [derive (Diagnostic)] # [diag (builtin_macros_asm_duplicate_arg)] pub (crate) struct AsmDuplicateArg { # [primary_span] # [label (builtin_macros_arg)] pub (crate) span : Span , # [label] pub (crate) prev : Span , pub (crate) name : Symbol , }
/* FP:errors.rs-0161 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0081
/* FP:errors.rs-0162 */ # [derive (Diagnostic)] # [diag (builtin_macros_asm_pos_after)] pub (crate) struct AsmPositionalAfter { # [primary_span] # [label (builtin_macros_pos)] pub (crate) span : Span , # [label (builtin_macros_named)] pub (crate) named : Vec < Span > , # [label (builtin_macros_explicit)] pub (crate) explicit : Vec < Span > , }
/* FP:errors.rs-0163 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0082
/* FP:errors.rs-0164 */ # [derive (Diagnostic)] # [diag (builtin_macros_asm_noreturn)] pub (crate) struct AsmNoReturn { # [primary_span] pub (crate) outputs_sp : Vec < Span > , }
/* FP:errors.rs-0165 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0083
/* FP:errors.rs-0166 */ # [derive (Diagnostic)] # [diag (builtin_macros_asm_no_matched_argument_name)] pub (crate) struct AsmNoMatchedArgumentName { pub (crate) name : String , # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0167 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0084
/* FP:errors.rs-0168 */ # [derive (Diagnostic)] # [diag (builtin_macros_asm_mayunwind)] pub (crate) struct AsmMayUnwind { # [primary_span] pub (crate) labels_sp : Vec < Span > , }
/* FP:errors.rs-0169 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0085
/* FP:errors.rs-0170 */ pub (crate) struct AsmClobberNoReg { pub (crate) spans : Vec < Span > , pub (crate) clobbers : Vec < Span > , }
/* FP:errors.rs-0171 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_IMPL_0086
/* FP:errors.rs-0172 */ impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for AsmClobberNoReg { fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let lbl1 = dcx . eagerly_translate_to_string (crate :: fluent_generated :: builtin_macros_asm_clobber_abi , [] . into_iter () ,) ; let lbl2 = dcx . eagerly_translate_to_string (crate :: fluent_generated :: builtin_macros_asm_clobber_outputs , [] . into_iter () ,) ; Diag :: new (dcx , level , crate :: fluent_generated :: builtin_macros_asm_clobber_no_reg) . with_span (self . spans . clone ()) . with_span_labels (self . clobbers , & lbl1) . with_span_labels (self . spans , & lbl2) } }
/* FP:errors.rs-0173 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0087
/* FP:errors.rs-0174 */ # [derive (Diagnostic)] # [diag (builtin_macros_asm_opt_already_provided)] pub (crate) struct AsmOptAlreadyprovided { # [primary_span] # [label] pub (crate) span : Span , pub (crate) symbol : Symbol , # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub (crate) span_with_comma : Span , }
/* FP:errors.rs-0175 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0088
/* FP:errors.rs-0176 */ # [derive (Diagnostic)] # [diag (builtin_macros_asm_unsupported_option)] pub (crate) struct AsmUnsupportedOption { # [primary_span] # [label] pub (crate) span : Span , pub (crate) symbol : Symbol , # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub (crate) span_with_comma : Span , pub (crate) macro_name : & 'static str , }
/* FP:errors.rs-0177 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0089
/* FP:errors.rs-0178 */ # [derive (Diagnostic)] # [diag (builtin_macros_asm_unsupported_clobber_abi)] pub (crate) struct AsmUnsupportedClobberAbi { # [primary_span] pub (crate) spans : Vec < Span > , pub (crate) macro_name : & 'static str , }
/* FP:errors.rs-0179 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0090
/* FP:errors.rs-0180 */ # [derive (Diagnostic)] # [diag (builtin_macros_test_runner_invalid)] pub (crate) struct TestRunnerInvalid { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0181 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0091
/* FP:errors.rs-0182 */ # [derive (Diagnostic)] # [diag (builtin_macros_test_runner_nargs)] pub (crate) struct TestRunnerNargs { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0183 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0092
/* FP:errors.rs-0184 */ # [derive (Diagnostic)] # [diag (builtin_macros_expected_comma_in_list)] pub (crate) struct ExpectedCommaInList { # [primary_span] pub span : Span , }
/* FP:errors.rs-0185 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0093
/* FP:errors.rs-0186 */ # [derive (Diagnostic)] # [diag (builtin_macros_only_one_argument)] pub (crate) struct OnlyOneArgument < 'a > { # [primary_span] pub span : Span , pub name : & 'a str , }
/* FP:errors.rs-0187 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0094
/* FP:errors.rs-0188 */ # [derive (Diagnostic)] # [diag (builtin_macros_takes_no_arguments)] pub (crate) struct TakesNoArguments < 'a > { # [primary_span] pub span : Span , pub name : & 'a str , }
/* FP:errors.rs-0189 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0095
/* FP:errors.rs-0190 */ # [derive (Diagnostic)] # [diag (builtin_macros_proc_macro_attribute_only_usable_with_crate_type)] pub (crate) struct AttributeOnlyUsableWithCrateType < 'a > { # [primary_span] pub span : Span , pub path : & 'a str , }
/* FP:errors.rs-0191 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0096
/* FP:errors.rs-0192 */ # [derive (Diagnostic)] # [diag (builtin_macros_source_uitls_expected_item)] pub (crate) struct ExpectedItem < 'a > { # [primary_span] pub span : Span , pub token : & 'a str , }
/* FP:errors.rs-0193 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0097
/* FP:errors.rs-0194 */ # [derive (Diagnostic)] # [diag (builtin_macros_naked_functions_testing_attribute , code = E0736)] pub (crate) struct NakedFunctionTestingAttribute { # [primary_span] # [label (builtin_macros_naked_attribute)] pub naked_span : Span , # [label] pub testing_span : Span , }
/* FP:errors.rs-0195 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0098
/* FP:errors.rs-0196 */ # [derive (Diagnostic)] # [diag (builtin_macros_non_generic_pointee)] pub (crate) struct NonGenericPointee { # [primary_span] pub span : Span , }
/* FP:errors.rs-0197 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0099
/* FP:errors.rs-0198 */ # [derive (Diagnostic)] # [diag (builtin_macros_expected_other)] pub (crate) struct AsmExpectedOther { # [primary_span] # [label (builtin_macros_expected_other)] pub (crate) span : Span , pub (crate) is_inline_asm : bool , }
/* FP:errors.rs-0199 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0100
/* FP:errors.rs-0200 */ # [derive (Diagnostic)] # [diag (builtin_macros_cfg_select_no_matches)] pub (crate) struct CfgSelectNoMatches { # [primary_span] pub span : Span , }
/* FP:errors.rs-0201 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_errors_STRUCT_0101
/* FP:errors.rs-0202 */ # [derive (Diagnostic)] # [diag (builtin_macros_cfg_select_unreachable)] pub (crate) struct CfgSelectUnreachable { # [primary_span] # [label (builtin_macros_label2)] pub span : Span , # [label] pub wildcard_span : Span , }