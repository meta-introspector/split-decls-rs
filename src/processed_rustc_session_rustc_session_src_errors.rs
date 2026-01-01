/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_USE_0001
/* FP:errors.rs-0002 */ use std :: num :: { NonZero , ParseIntError } ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_USE_0002
/* FP:errors.rs-0004 */ use crate :: rustc_complete :: token ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_USE_0003
/* FP:errors.rs-0006 */ use crate :: rustc_complete :: util :: literal :: LitError ;
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_USE_0004
/* FP:errors.rs-0008 */ use crate :: rustc_complete :: codes :: * ;
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_USE_0005
/* FP:errors.rs-0010 */ use crate :: rustc_complete :: { Diag , DiagCtxtHandle , DiagMessage , Diagnostic , EmissionGuarantee , ErrorGuaranteed , Level , MultiSpan , } ;
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_USE_0006
/* FP:errors.rs-0012 */ use rustc_macros :: { Diagnostic , LintDiagnostic , Subdiagnostic } ;
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_USE_0007
/* FP:errors.rs-0014 */ use crate :: rustc_complete :: { Span , Symbol } ;
/* FP:errors.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_USE_0008
/* FP:errors.rs-0016 */ use crate :: rustc_target :: spec :: { SplitDebuginfo , StackProtector , TargetTuple } ;
/* FP:errors.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_USE_0009
/* FP:errors.rs-0018 */ use crate :: config :: CrateType ;
/* FP:errors.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_USE_0010
/* FP:errors.rs-0020 */ use crate :: parse :: ParseSess ;
/* FP:errors.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_ENUM_0011
/* FP:errors.rs-0022 */ # [derive (Diagnostic)] pub (crate) enum AppleDeploymentTarget { # [diag (session_apple_deployment_target_invalid)] Invalid { env_var : & 'static str , error : ParseIntError } , # [diag (session_apple_deployment_target_too_low)] TooLow { env_var : & 'static str , version : String , os_min : String } , }
/* FP:errors.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0012
/* FP:errors.rs-0024 */ pub (crate) struct FeatureGateError { pub (crate) span : MultiSpan , pub (crate) explain : DiagMessage , }
/* FP:errors.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_IMPL_0013
/* FP:errors.rs-0026 */ impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for FeatureGateError { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { Diag :: new (dcx , level , self . explain) . with_span (self . span) . with_code (E0658) } }
/* FP:errors.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0014
/* FP:errors.rs-0028 */ # [derive (Subdiagnostic)] # [note (session_feature_diagnostic_for_issue)] pub (crate) struct FeatureDiagnosticForIssue { pub (crate) n : NonZero < u32 > , }
/* FP:errors.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0015
/* FP:errors.rs-0030 */ # [derive (Subdiagnostic)] # [note (session_feature_suggest_upgrade_compiler)] pub (crate) struct SuggestUpgradeCompiler { date : & 'static str , }
/* FP:errors.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_IMPL_0016
/* FP:errors.rs-0032 */ impl SuggestUpgradeCompiler { pub (crate) fn ui_testing () -> Self { Self { date : "YYYY-MM-DD" } } pub (crate) fn new () -> Option < Self > { let date = option_env ! ("CFG_VER_DATE") ? ; Some (Self { date }) } }
/* FP:errors.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0017
/* FP:errors.rs-0034 */ # [derive (Subdiagnostic)] # [help (session_feature_diagnostic_help)] pub (crate) struct FeatureDiagnosticHelp { pub (crate) feature : Symbol , }
/* FP:errors.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0018
/* FP:errors.rs-0036 */ # [derive (Subdiagnostic)] # [suggestion (session_feature_diagnostic_suggestion , applicability = "maybe-incorrect" , code = "#[feature({feature})]\n")] pub struct FeatureDiagnosticSuggestion { pub feature : Symbol , # [primary_span] pub span : Span , }
/* FP:errors.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0019
/* FP:errors.rs-0038 */ # [derive (Subdiagnostic)] # [help (session_cli_feature_diagnostic_help)] pub (crate) struct CliFeatureDiagnosticHelp { pub (crate) feature : Symbol , }
/* FP:errors.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0020
/* FP:errors.rs-0040 */ # [derive (Diagnostic)] # [diag (session_not_circumvent_feature)] pub (crate) struct NotCircumventFeature ;
/* FP:errors.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0021
/* FP:errors.rs-0042 */ # [derive (Diagnostic)] # [diag (session_linker_plugin_lto_windows_not_supported)] pub (crate) struct LinkerPluginToWindowsNotSupported ;
/* FP:errors.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0022
/* FP:errors.rs-0044 */ # [derive (Diagnostic)] # [diag (session_profile_use_file_does_not_exist)] pub (crate) struct ProfileUseFileDoesNotExist < 'a > { pub (crate) path : & 'a std :: path :: Path , }
/* FP:errors.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0023
/* FP:errors.rs-0046 */ # [derive (Diagnostic)] # [diag (session_profile_sample_use_file_does_not_exist)] pub (crate) struct ProfileSampleUseFileDoesNotExist < 'a > { pub (crate) path : & 'a std :: path :: Path , }
/* FP:errors.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0024
/* FP:errors.rs-0048 */ # [derive (Diagnostic)] # [diag (session_target_requires_unwind_tables)] pub (crate) struct TargetRequiresUnwindTables ;
/* FP:errors.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0025
/* FP:errors.rs-0050 */ # [derive (Diagnostic)] # [diag (session_instrumentation_not_supported)] pub (crate) struct InstrumentationNotSupported { pub (crate) us : String , }
/* FP:errors.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0026
/* FP:errors.rs-0052 */ # [derive (Diagnostic)] # [diag (session_sanitizer_not_supported)] pub (crate) struct SanitizerNotSupported { pub (crate) us : String , }
/* FP:errors.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0027
/* FP:errors.rs-0054 */ # [derive (Diagnostic)] # [diag (session_sanitizers_not_supported)] pub (crate) struct SanitizersNotSupported { pub (crate) us : String , }
/* FP:errors.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0028
/* FP:errors.rs-0056 */ # [derive (Diagnostic)] # [diag (session_cannot_mix_and_match_sanitizers)] pub (crate) struct CannotMixAndMatchSanitizers { pub (crate) first : String , pub (crate) second : String , }
/* FP:errors.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0029
/* FP:errors.rs-0058 */ # [derive (Diagnostic)] # [diag (session_cannot_enable_crt_static_linux)] pub (crate) struct CannotEnableCrtStaticLinux ;
/* FP:errors.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0030
/* FP:errors.rs-0060 */ # [derive (Diagnostic)] # [diag (session_sanitizer_cfi_requires_lto)] pub (crate) struct SanitizerCfiRequiresLto ;
/* FP:errors.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0031
/* FP:errors.rs-0062 */ # [derive (Diagnostic)] # [diag (session_sanitizer_cfi_requires_single_codegen_unit)] pub (crate) struct SanitizerCfiRequiresSingleCodegenUnit ;
/* FP:errors.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0032
/* FP:errors.rs-0064 */ # [derive (Diagnostic)] # [diag (session_sanitizer_cfi_canonical_jump_tables_requires_cfi)] pub (crate) struct SanitizerCfiCanonicalJumpTablesRequiresCfi ;
/* FP:errors.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0033
/* FP:errors.rs-0066 */ # [derive (Diagnostic)] # [diag (session_sanitizer_cfi_generalize_pointers_requires_cfi)] pub (crate) struct SanitizerCfiGeneralizePointersRequiresCfi ;
/* FP:errors.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0034
/* FP:errors.rs-0068 */ # [derive (Diagnostic)] # [diag (session_sanitizer_cfi_normalize_integers_requires_cfi)] pub (crate) struct SanitizerCfiNormalizeIntegersRequiresCfi ;
/* FP:errors.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0035
/* FP:errors.rs-0070 */ # [derive (Diagnostic)] # [diag (session_sanitizer_kcfi_arity_requires_kcfi)] pub (crate) struct SanitizerKcfiArityRequiresKcfi ;
/* FP:errors.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0036
/* FP:errors.rs-0072 */ # [derive (Diagnostic)] # [diag (session_sanitizer_kcfi_requires_panic_abort)] pub (crate) struct SanitizerKcfiRequiresPanicAbort ;
/* FP:errors.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0037
/* FP:errors.rs-0074 */ # [derive (Diagnostic)] # [diag (session_split_lto_unit_requires_lto)] pub (crate) struct SplitLtoUnitRequiresLto ;
/* FP:errors.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0038
/* FP:errors.rs-0076 */ # [derive (Diagnostic)] # [diag (session_unstable_virtual_function_elimination)] pub (crate) struct UnstableVirtualFunctionElimination ;
/* FP:errors.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0039
/* FP:errors.rs-0078 */ # [derive (Diagnostic)] # [diag (session_unsupported_dwarf_version)] # [help (session_unsupported_dwarf_version_help)] pub (crate) struct UnsupportedDwarfVersion { pub (crate) dwarf_version : u32 , }
/* FP:errors.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0040
/* FP:errors.rs-0080 */ # [derive (Diagnostic)] # [diag (session_embed_source_insufficient_dwarf_version)] pub (crate) struct EmbedSourceInsufficientDwarfVersion { pub (crate) dwarf_version : u32 , }
/* FP:errors.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0041
/* FP:errors.rs-0082 */ # [derive (Diagnostic)] # [diag (session_embed_source_requires_debug_info)] pub (crate) struct EmbedSourceRequiresDebugInfo ;
/* FP:errors.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0042
/* FP:errors.rs-0084 */ # [derive (Diagnostic)] # [diag (session_target_stack_protector_not_supported)] pub (crate) struct StackProtectorNotSupportedForTarget < 'a > { pub (crate) stack_protector : StackProtector , pub (crate) target_triple : & 'a TargetTuple , }
/* FP:errors.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0043
/* FP:errors.rs-0086 */ # [derive (Diagnostic)] # [diag (session_target_small_data_threshold_not_supported)] pub (crate) struct SmallDataThresholdNotSupportedForTarget < 'a > { pub (crate) target_triple : & 'a TargetTuple , }
/* FP:errors.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0044
/* FP:errors.rs-0088 */ # [derive (Diagnostic)] # [diag (session_branch_protection_requires_aarch64)] pub (crate) struct BranchProtectionRequiresAArch64 ;
/* FP:errors.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0045
/* FP:errors.rs-0090 */ # [derive (Diagnostic)] # [diag (session_split_debuginfo_unstable_platform)] pub (crate) struct SplitDebugInfoUnstablePlatform { pub (crate) debuginfo : SplitDebuginfo , }
/* FP:errors.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0046
/* FP:errors.rs-0092 */ # [derive (Diagnostic)] # [diag (session_file_is_not_writeable)] pub (crate) struct FileIsNotWriteable < 'a > { pub (crate) file : & 'a std :: path :: Path , }
/* FP:errors.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0047
/* FP:errors.rs-0094 */ # [derive (Diagnostic)] # [diag (session_file_write_fail)] pub (crate) struct FileWriteFail < 'a > { pub (crate) path : & 'a std :: path :: Path , pub (crate) err : String , }
/* FP:errors.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0048
/* FP:errors.rs-0096 */ # [derive (Diagnostic)] # [diag (session_crate_name_empty)] pub (crate) struct CrateNameEmpty { # [primary_span] pub (crate) span : Option < Span > , }
/* FP:errors.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0049
/* FP:errors.rs-0098 */ # [derive (Diagnostic)] # [diag (session_invalid_character_in_crate_name)] pub (crate) struct InvalidCharacterInCrateName { # [primary_span] pub (crate) span : Option < Span > , pub (crate) character : char , pub (crate) crate_name : Symbol , # [help] pub (crate) help : Option < () > , }
/* FP:errors.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0050
/* FP:errors.rs-0100 */ # [derive (Subdiagnostic)] # [multipart_suggestion (session_expr_parentheses_needed , applicability = "machine-applicable")] pub struct ExprParenthesesNeeded { # [suggestion_part (code = "(")] left : Span , # [suggestion_part (code = ")")] right : Span , }
/* FP:errors.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_IMPL_0051
/* FP:errors.rs-0102 */ impl ExprParenthesesNeeded { pub fn surrounding (s : Span) -> Self { ExprParenthesesNeeded { left : s . shrink_to_lo () , right : s . shrink_to_hi () } } }
/* FP:errors.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0052
/* FP:errors.rs-0104 */ # [derive (Diagnostic)] # [diag (session_skipping_const_checks)] pub (crate) struct SkippingConstChecks { # [subdiagnostic] pub (crate) unleashed_features : Vec < UnleashedFeatureHelp > , }
/* FP:errors.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_ENUM_0053
/* FP:errors.rs-0106 */ # [derive (Subdiagnostic)] pub (crate) enum UnleashedFeatureHelp { # [help (session_unleashed_feature_help_named)] Named { # [primary_span] span : Span , gate : Symbol , } , # [help (session_unleashed_feature_help_unnamed)] Unnamed { # [primary_span] span : Span , } , }
/* FP:errors.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0054
/* FP:errors.rs-0108 */ # [derive (Diagnostic)] # [diag (session_invalid_literal_suffix)] struct InvalidLiteralSuffix < 'a > { # [primary_span] # [label] span : Span , kind : & 'a str , suffix : Symbol , }
/* FP:errors.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0055
/* FP:errors.rs-0110 */ # [derive (Diagnostic)] # [diag (session_invalid_int_literal_width)] # [help] struct InvalidIntLiteralWidth { # [primary_span] span : Span , width : String , }
/* FP:errors.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0056
/* FP:errors.rs-0112 */ # [derive (Diagnostic)] # [diag (session_invalid_num_literal_base_prefix)] # [note] struct InvalidNumLiteralBasePrefix { # [primary_span] # [suggestion (applicability = "maybe-incorrect" , code = "{fixed}")] span : Span , fixed : String , }
/* FP:errors.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0057
/* FP:errors.rs-0114 */ # [derive (Diagnostic)] # [diag (session_invalid_num_literal_suffix)] # [help] struct InvalidNumLiteralSuffix { # [primary_span] # [label] span : Span , suffix : String , }
/* FP:errors.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0058
/* FP:errors.rs-0116 */ # [derive (Diagnostic)] # [diag (session_invalid_float_literal_width)] # [help] struct InvalidFloatLiteralWidth { # [primary_span] span : Span , width : String , }
/* FP:errors.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0059
/* FP:errors.rs-0118 */ # [derive (Diagnostic)] # [diag (session_invalid_float_literal_suffix)] # [help] struct InvalidFloatLiteralSuffix { # [primary_span] # [label] span : Span , suffix : String , }
/* FP:errors.rs-0119 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0060
/* FP:errors.rs-0120 */ # [derive (Diagnostic)] # [diag (session_int_literal_too_large)] # [note] struct IntLiteralTooLarge { # [primary_span] span : Span , limit : String , }
/* FP:errors.rs-0121 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0061
/* FP:errors.rs-0122 */ # [derive (Diagnostic)] # [diag (session_hexadecimal_float_literal_not_supported)] struct HexadecimalFloatLiteralNotSupported { # [primary_span] # [label (session_not_supported)] span : Span , }
/* FP:errors.rs-0123 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0062
/* FP:errors.rs-0124 */ # [derive (Diagnostic)] # [diag (session_octal_float_literal_not_supported)] struct OctalFloatLiteralNotSupported { # [primary_span] # [label (session_not_supported)] span : Span , }
/* FP:errors.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0063
/* FP:errors.rs-0126 */ # [derive (Diagnostic)] # [diag (session_binary_float_literal_not_supported)] struct BinaryFloatLiteralNotSupported { # [primary_span] # [label (session_not_supported)] span : Span , }
/* FP:errors.rs-0127 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0064
/* FP:errors.rs-0128 */ # [derive (Diagnostic)] # [diag (session_unsupported_crate_type_for_target)] pub (crate) struct UnsupportedCrateTypeForTarget < 'a > { pub (crate) crate_type : CrateType , pub (crate) target_triple : & 'a TargetTuple , }
/* FP:errors.rs-0129 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_FN_0065
/* FP:errors.rs-0130 */ pub fn report_lit_error (psess : & ParseSess , err : LitError , lit : token :: Lit , span : Span ,) -> ErrorGuaranteed { create_lit_error (psess , err , lit , span) . emit () }
/* FP:errors.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_FN_0066
/* FP:errors.rs-0132 */ pub fn create_lit_error (psess : & ParseSess , err : LitError , lit : token :: Lit , span : Span) -> Diag < '_ > { fn looks_like_width_suffix (first_chars : & [char] , s : & str) -> bool { s . len () > 1 && s . starts_with (first_chars) && s [1 ..] . chars () . all (| c | c . is_ascii_digit ()) } fn fix_base_capitalisation (prefix : & str , suffix : & str) -> Option < String > { let mut chars = suffix . chars () ; let base_char = chars . next () . unwrap () ; let base = match base_char { 'B' => 2 , 'O' => 8 , 'X' => 16 , _ => return None , } ; let valid = prefix == "0" && chars . filter (| c | * c != '_') . take_while (| c | * c != 'i' && * c != 'u') . all (| c | c . to_digit (base) . is_some ()) ; valid . then (| | format ! ("0{}{}" , base_char . to_ascii_lowercase () , & suffix [1 ..])) } let dcx = psess . dcx () ; match err { LitError :: InvalidSuffix (suffix) => { dcx . create_err (InvalidLiteralSuffix { span , kind : lit . kind . descr () , suffix }) } LitError :: InvalidIntSuffix (suffix) => { let suf = suffix . as_str () ; if looks_like_width_suffix (& ['i' , 'u'] , suf) { dcx . create_err (InvalidIntLiteralWidth { span , width : suf [1 ..] . into () }) } else if let Some (fixed) = fix_base_capitalisation (lit . symbol . as_str () , suf) { dcx . create_err (InvalidNumLiteralBasePrefix { span , fixed }) } else { dcx . create_err (InvalidNumLiteralSuffix { span , suffix : suf . to_string () }) } } LitError :: InvalidFloatSuffix (suffix) => { let suf = suffix . as_str () ; if looks_like_width_suffix (& ['f'] , suf) { dcx . create_err (InvalidFloatLiteralWidth { span , width : suf [1 ..] . to_string () }) } else { dcx . create_err (InvalidFloatLiteralSuffix { span , suffix : suf . to_string () }) } } LitError :: NonDecimalFloat (base) => match base { 16 => dcx . create_err (HexadecimalFloatLiteralNotSupported { span }) , 8 => dcx . create_err (OctalFloatLiteralNotSupported { span }) , 2 => dcx . create_err (BinaryFloatLiteralNotSupported { span }) , _ => unreachable ! () , } , LitError :: IntTooLarge (base) => { let max = u128 :: MAX ; let limit = match base { 2 => format ! ("{max:#b}") , 8 => format ! ("{max:#o}") , 16 => format ! ("{max:#x}") , _ => format ! ("{max}") , } ; dcx . create_err (IntLiteralTooLarge { span , limit }) } } }
/* FP:errors.rs-0133 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0067
/* FP:errors.rs-0134 */ # [derive (Diagnostic)] # [diag (session_incompatible_linker_flavor)] # [note] pub (crate) struct IncompatibleLinkerFlavor { pub (crate) flavor : & 'static str , pub (crate) compatible_list : String , }
/* FP:errors.rs-0135 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0068
/* FP:errors.rs-0136 */ # [derive (Diagnostic)] # [diag (session_function_return_requires_x86_or_x86_64)] pub (crate) struct FunctionReturnRequiresX86OrX8664 ;
/* FP:errors.rs-0137 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0069
/* FP:errors.rs-0138 */ # [derive (Diagnostic)] # [diag (session_function_return_thunk_extern_requires_non_large_code_model)] pub (crate) struct FunctionReturnThunkExternRequiresNonLargeCodeModel ;
/* FP:errors.rs-0139 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0070
/* FP:errors.rs-0140 */ # [derive (Diagnostic)] # [diag (session_indirect_branch_cs_prefix_requires_x86_or_x86_64)] pub (crate) struct IndirectBranchCsPrefixRequiresX86OrX8664 ;
/* FP:errors.rs-0141 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0071
/* FP:errors.rs-0142 */ # [derive (Diagnostic)] # [diag (session_unsupported_regparm)] pub (crate) struct UnsupportedRegparm { pub (crate) regparm : u32 , }
/* FP:errors.rs-0143 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0072
/* FP:errors.rs-0144 */ # [derive (Diagnostic)] # [diag (session_unsupported_regparm_arch)] pub (crate) struct UnsupportedRegparmArch ;
/* FP:errors.rs-0145 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0073
/* FP:errors.rs-0146 */ # [derive (Diagnostic)] # [diag (session_unsupported_reg_struct_return_arch)] pub (crate) struct UnsupportedRegStructReturnArch ;
/* FP:errors.rs-0147 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0074
/* FP:errors.rs-0148 */ # [derive (Diagnostic)] # [diag (session_failed_to_create_profiler)] pub (crate) struct FailedToCreateProfiler { pub (crate) err : String , }
/* FP:errors.rs-0149 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0075
/* FP:errors.rs-0150 */ # [derive (Diagnostic)] # [diag (session_soft_float_ignored)] # [note] pub (crate) struct SoftFloatIgnored ;
/* FP:errors.rs-0151 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0076
/* FP:errors.rs-0152 */ # [derive (Diagnostic)] # [diag (session_soft_float_deprecated)] # [note] # [note (session_soft_float_deprecated_issue)] pub (crate) struct SoftFloatDeprecated ;
/* FP:errors.rs-0153 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_errors_STRUCT_0077
/* FP:errors.rs-0154 */ # [derive (LintDiagnostic)] # [diag (session_unexpected_builtin_cfg)] # [note (session_controlled_by)] # [note (session_incoherent)] pub (crate) struct UnexpectedBuiltinCfg { pub (crate) cfg : String , pub (crate) cfg_name : Symbol , pub (crate) controlled_by : & 'static str , }