/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_USE_0001
/* FP:errors.rs-0002 */ use crate :: rustc_complete :: codes :: * ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_USE_0002
/* FP:errors.rs-0004 */ use crate :: rustc_complete :: { Applicability , Diag , ElidedLifetimeInPathSubdiag , EmissionGuarantee , IntoDiagArg , MultiSpan , Subdiagnostic , } ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_USE_0003
/* FP:errors.rs-0006 */ use rustc_macros :: { Diagnostic , Subdiagnostic } ;
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_USE_0004
/* FP:errors.rs-0008 */ use crate :: rustc_complete :: { Ident , Span , Symbol } ;
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_USE_0005
/* FP:errors.rs-0010 */ use crate :: late :: PatternSource ;
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_USE_0006
/* FP:errors.rs-0012 */ use crate :: { Res , fluent_generated as fluent } ;
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0007
/* FP:errors.rs-0014 */ # [derive (Diagnostic)] # [diag (resolve_generic_params_from_outer_item , code = E0401)] pub (crate) struct GenericParamsFromOuterItem { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) label : Option < GenericParamsFromOuterItemLabel > , # [label (resolve_refer_to_type_directly)] pub (crate) refer_to_type_directly : Option < Span > , # [subdiagnostic] pub (crate) sugg : Option < GenericParamsFromOuterItemSugg > , # [subdiagnostic] pub (crate) static_or_const : Option < GenericParamsFromOuterItemStaticOrConst > , pub (crate) is_self : bool , }
/* FP:errors.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_ENUM_0008
/* FP:errors.rs-0016 */ # [derive (Subdiagnostic)] pub (crate) enum GenericParamsFromOuterItemStaticOrConst { # [note (resolve_generic_params_from_outer_item_static)] Static , # [note (resolve_generic_params_from_outer_item_const)] Const , }
/* FP:errors.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_ENUM_0009
/* FP:errors.rs-0018 */ # [derive (Subdiagnostic)] pub (crate) enum GenericParamsFromOuterItemLabel { # [label (resolve_generic_params_from_outer_item_self_ty_param)] SelfTyParam (# [primary_span] Span) , # [label (resolve_generic_params_from_outer_item_self_ty_alias)] SelfTyAlias (# [primary_span] Span) , # [label (resolve_generic_params_from_outer_item_ty_param)] TyParam (# [primary_span] Span) , # [label (resolve_generic_params_from_outer_item_const_param)] ConstParam (# [primary_span] Span) , }
/* FP:errors.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0010
/* FP:errors.rs-0020 */ # [derive (Subdiagnostic)] # [suggestion (resolve_suggestion , code = "{snippet}" , applicability = "maybe-incorrect")] pub (crate) struct GenericParamsFromOuterItemSugg { # [primary_span] pub (crate) span : Span , pub (crate) snippet : String , }
/* FP:errors.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0011
/* FP:errors.rs-0022 */ # [derive (Diagnostic)] # [diag (resolve_name_is_already_used_as_generic_parameter , code = E0403)] pub (crate) struct NameAlreadyUsedInParameterList { # [primary_span] # [label] pub (crate) span : Span , # [label (resolve_first_use_of_name)] pub (crate) first_use_span : Span , pub (crate) name : Ident , }
/* FP:errors.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0012
/* FP:errors.rs-0024 */ # [derive (Diagnostic)] # [diag (resolve_method_not_member_of_trait , code = E0407)] pub (crate) struct MethodNotMemberOfTrait { # [primary_span] # [label] pub (crate) span : Span , pub (crate) method : Ident , pub (crate) trait_ : String , # [subdiagnostic] pub (crate) sub : Option < AssociatedFnWithSimilarNameExists > , }
/* FP:errors.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0013
/* FP:errors.rs-0026 */ # [derive (Subdiagnostic)] # [suggestion (resolve_associated_fn_with_similar_name_exists , code = "{candidate}" , applicability = "maybe-incorrect")] pub (crate) struct AssociatedFnWithSimilarNameExists { # [primary_span] pub (crate) span : Span , pub (crate) candidate : Symbol , }
/* FP:errors.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0014
/* FP:errors.rs-0028 */ # [derive (Diagnostic)] # [diag (resolve_type_not_member_of_trait , code = E0437)] pub (crate) struct TypeNotMemberOfTrait { # [primary_span] # [label] pub (crate) span : Span , pub (crate) type_ : Ident , pub (crate) trait_ : String , # [subdiagnostic] pub (crate) sub : Option < AssociatedTypeWithSimilarNameExists > , }
/* FP:errors.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0015
/* FP:errors.rs-0030 */ # [derive (Subdiagnostic)] # [suggestion (resolve_associated_type_with_similar_name_exists , code = "{candidate}" , applicability = "maybe-incorrect")] pub (crate) struct AssociatedTypeWithSimilarNameExists { # [primary_span] pub (crate) span : Span , pub (crate) candidate : Symbol , }
/* FP:errors.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0016
/* FP:errors.rs-0032 */ # [derive (Diagnostic)] # [diag (resolve_const_not_member_of_trait , code = E0438)] pub (crate) struct ConstNotMemberOfTrait { # [primary_span] # [label] pub (crate) span : Span , pub (crate) const_ : Ident , pub (crate) trait_ : String , # [subdiagnostic] pub (crate) sub : Option < AssociatedConstWithSimilarNameExists > , }
/* FP:errors.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0017
/* FP:errors.rs-0034 */ # [derive (Subdiagnostic)] # [suggestion (resolve_associated_const_with_similar_name_exists , code = "{candidate}" , applicability = "maybe-incorrect")] pub (crate) struct AssociatedConstWithSimilarNameExists { # [primary_span] pub (crate) span : Span , pub (crate) candidate : Symbol , }
/* FP:errors.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0018
/* FP:errors.rs-0036 */ # [derive (Diagnostic)] # [diag (resolve_variable_bound_with_different_mode , code = E0409)] pub (crate) struct VariableBoundWithDifferentMode { # [primary_span] # [label] pub (crate) span : Span , # [label (resolve_first_binding_span)] pub (crate) first_binding_span : Span , pub (crate) variable_name : Ident , }
/* FP:errors.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0019
/* FP:errors.rs-0038 */ # [derive (Diagnostic)] # [diag (resolve_ident_bound_more_than_once_in_parameter_list , code = E0415)] pub (crate) struct IdentifierBoundMoreThanOnceInParameterList { # [primary_span] # [label] pub (crate) span : Span , pub (crate) identifier : Ident , }
/* FP:errors.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0020
/* FP:errors.rs-0040 */ # [derive (Diagnostic)] # [diag (resolve_ident_bound_more_than_once_in_same_pattern , code = E0416)] pub (crate) struct IdentifierBoundMoreThanOnceInSamePattern { # [primary_span] # [label] pub (crate) span : Span , pub (crate) identifier : Ident , }
/* FP:errors.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0021
/* FP:errors.rs-0042 */ # [derive (Diagnostic)] # [diag (resolve_undeclared_label , code = E0426)] pub (crate) struct UndeclaredLabel { # [primary_span] # [label] pub (crate) span : Span , pub (crate) name : Symbol , # [subdiagnostic] pub (crate) sub_reachable : Option < LabelWithSimilarNameReachable > , # [subdiagnostic] pub (crate) sub_reachable_suggestion : Option < TryUsingSimilarlyNamedLabel > , # [subdiagnostic] pub (crate) sub_unreachable : Option < UnreachableLabelWithSimilarNameExists > , }
/* FP:errors.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0022
/* FP:errors.rs-0044 */ # [derive (Subdiagnostic)] # [label (resolve_label_with_similar_name_reachable)] pub (crate) struct LabelWithSimilarNameReachable (# [primary_span] pub (crate) Span) ;
/* FP:errors.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0023
/* FP:errors.rs-0046 */ # [derive (Subdiagnostic)] # [suggestion (resolve_try_using_similarly_named_label , code = "{ident_name}" , applicability = "maybe-incorrect")] pub (crate) struct TryUsingSimilarlyNamedLabel { # [primary_span] pub (crate) span : Span , pub (crate) ident_name : Symbol , }
/* FP:errors.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0024
/* FP:errors.rs-0048 */ # [derive (Subdiagnostic)] # [label (resolve_unreachable_label_with_similar_name_exists)] pub (crate) struct UnreachableLabelWithSimilarNameExists { # [primary_span] pub (crate) ident_span : Span , }
/* FP:errors.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0025
/* FP:errors.rs-0050 */ # [derive (Diagnostic)] # [diag (resolve_self_import_can_only_appear_once_in_the_list , code = E0430)] pub (crate) struct SelfImportCanOnlyAppearOnceInTheList { # [primary_span] # [label] pub (crate) span : Span , }
/* FP:errors.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0026
/* FP:errors.rs-0052 */ # [derive (Diagnostic)] # [diag (resolve_self_import_only_in_import_list_with_non_empty_prefix , code = E0431)] pub (crate) struct SelfImportOnlyInImportListWithNonEmptyPrefix { # [primary_span] # [label] pub (crate) span : Span , }
/* FP:errors.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0027
/* FP:errors.rs-0054 */ # [derive (Diagnostic)] # [diag (resolve_cannot_capture_dynamic_environment_in_fn_item , code = E0434)] # [help] pub (crate) struct CannotCaptureDynamicEnvironmentInFnItem { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0028
/* FP:errors.rs-0056 */ # [derive (Diagnostic)] # [diag (resolve_attempt_to_use_non_constant_value_in_constant , code = E0435)] pub (crate) struct AttemptToUseNonConstantValueInConstant < 'a > { # [primary_span] pub (crate) span : Span , # [subdiagnostic] pub (crate) with : Option < AttemptToUseNonConstantValueInConstantWithSuggestion < 'a > > , # [subdiagnostic] pub (crate) with_label : Option < AttemptToUseNonConstantValueInConstantLabelWithSuggestion > , # [subdiagnostic] pub (crate) without : Option < AttemptToUseNonConstantValueInConstantWithoutSuggestion < 'a > > , }
/* FP:errors.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0029
/* FP:errors.rs-0058 */ # [derive (Subdiagnostic)] # [multipart_suggestion (resolve_attempt_to_use_non_constant_value_in_constant_with_suggestion , style = "verbose" , applicability = "has-placeholders")] pub (crate) struct AttemptToUseNonConstantValueInConstantWithSuggestion < 'a > { # [suggestion_part (code = "{suggestion} ")] pub (crate) span : Span , pub (crate) suggestion : & 'a str , # [suggestion_part (code = ": /* Type */")] pub (crate) type_span : Option < Span > , pub (crate) current : & 'a str , }
/* FP:errors.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0030
/* FP:errors.rs-0060 */ # [derive (Subdiagnostic)] # [label (resolve_attempt_to_use_non_constant_value_in_constant_label_with_suggestion)] pub (crate) struct AttemptToUseNonConstantValueInConstantLabelWithSuggestion { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0031
/* FP:errors.rs-0062 */ # [derive (Subdiagnostic)] # [label (resolve_attempt_to_use_non_constant_value_in_constant_without_suggestion)] pub (crate) struct AttemptToUseNonConstantValueInConstantWithoutSuggestion < 'a > { # [primary_span] pub (crate) ident_span : Span , pub (crate) suggestion : & 'a str , }
/* FP:errors.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0032
/* FP:errors.rs-0064 */ # [derive (Diagnostic)] # [diag (resolve_self_imports_only_allowed_within , code = E0429)] pub (crate) struct SelfImportsOnlyAllowedWithin { # [primary_span] pub (crate) span : Span , # [subdiagnostic] pub (crate) suggestion : Option < SelfImportsOnlyAllowedWithinSuggestion > , # [subdiagnostic] pub (crate) mpart_suggestion : Option < SelfImportsOnlyAllowedWithinMultipartSuggestion > , }
/* FP:errors.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0033
/* FP:errors.rs-0066 */ # [derive (Subdiagnostic)] # [suggestion (resolve_self_imports_only_allowed_within_suggestion , code = "" , applicability = "machine-applicable")] pub (crate) struct SelfImportsOnlyAllowedWithinSuggestion { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0034
/* FP:errors.rs-0068 */ # [derive (Subdiagnostic)] # [multipart_suggestion (resolve_self_imports_only_allowed_within_multipart_suggestion , applicability = "machine-applicable")] pub (crate) struct SelfImportsOnlyAllowedWithinMultipartSuggestion { # [suggestion_part (code = "{{")] pub (crate) multipart_start : Span , # [suggestion_part (code = "}}")] pub (crate) multipart_end : Span , }
/* FP:errors.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0035
/* FP:errors.rs-0070 */ # [derive (Diagnostic)] # [diag (resolve_binding_shadows_something_unacceptable , code = E0530)] pub (crate) struct BindingShadowsSomethingUnacceptable < 'a > { # [primary_span] # [label] pub (crate) span : Span , pub (crate) shadowing_binding : PatternSource , pub (crate) shadowed_binding : Res , pub (crate) article : & 'a str , # [subdiagnostic] pub (crate) sub_suggestion : Option < BindingShadowsSomethingUnacceptableSuggestion > , # [label (resolve_label_shadowed_binding)] pub (crate) shadowed_binding_span : Span , pub (crate) participle : & 'a str , pub (crate) name : Symbol , }
/* FP:errors.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0036
/* FP:errors.rs-0072 */ # [derive (Subdiagnostic)] # [suggestion (resolve_binding_shadows_something_unacceptable_suggestion , code = "{name}(..)" , applicability = "unspecified")] pub (crate) struct BindingShadowsSomethingUnacceptableSuggestion { # [primary_span] pub (crate) span : Span , pub (crate) name : Symbol , }
/* FP:errors.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0037
/* FP:errors.rs-0074 */ # [derive (Diagnostic)] # [diag (resolve_forward_declared_generic_param , code = E0128)] pub (crate) struct ForwardDeclaredGenericParam { # [primary_span] # [label] pub (crate) span : Span , pub (crate) param : Symbol , }
/* FP:errors.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0038
/* FP:errors.rs-0076 */ # [derive (Diagnostic)] # [diag (resolve_forward_declared_generic_in_const_param_ty)] pub (crate) struct ForwardDeclaredGenericInConstParamTy { # [primary_span] # [label] pub (crate) span : Span , pub (crate) param : Symbol , }
/* FP:errors.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0039
/* FP:errors.rs-0078 */ # [derive (Diagnostic)] # [diag (resolve_param_in_ty_of_const_param , code = E0770)] pub (crate) struct ParamInTyOfConstParam { # [primary_span] # [label] pub (crate) span : Span , pub (crate) name : Symbol , }
/* FP:errors.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0040
/* FP:errors.rs-0080 */ # [derive (Diagnostic)] # [diag (resolve_self_in_generic_param_default , code = E0735)] pub (crate) struct SelfInGenericParamDefault { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0041
/* FP:errors.rs-0082 */ # [derive (Diagnostic)] # [diag (resolve_self_in_const_generic_ty)] pub (crate) struct SelfInConstGenericTy { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0042
/* FP:errors.rs-0084 */ # [derive (Diagnostic)] # [diag (resolve_param_in_non_trivial_anon_const)] pub (crate) struct ParamInNonTrivialAnonConst { # [primary_span] # [label] pub (crate) span : Span , pub (crate) name : Symbol , # [subdiagnostic] pub (crate) param_kind : ParamKindInNonTrivialAnonConst , # [subdiagnostic] pub (crate) help : Option < ParamInNonTrivialAnonConstHelp > , }
/* FP:errors.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0043
/* FP:errors.rs-0086 */ # [derive (Subdiagnostic)] # [help (resolve_param_in_non_trivial_anon_const_help)] pub (crate) struct ParamInNonTrivialAnonConstHelp ;
/* FP:errors.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_ENUM_0044
/* FP:errors.rs-0088 */ # [derive (Debug)] # [derive (Subdiagnostic)] pub (crate) enum ParamKindInNonTrivialAnonConst { # [note (resolve_type_param_in_non_trivial_anon_const)] Type , # [help (resolve_const_param_in_non_trivial_anon_const)] Const { name : Symbol } , # [note (resolve_lifetime_param_in_non_trivial_anon_const)] Lifetime , }
/* FP:errors.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0045
/* FP:errors.rs-0090 */ # [derive (Diagnostic)] # [diag (resolve_unreachable_label , code = E0767)] # [note] pub (crate) struct UnreachableLabel { # [primary_span] # [label] pub (crate) span : Span , pub (crate) name : Symbol , # [label (resolve_label_definition_span)] pub (crate) definition_span : Span , # [subdiagnostic] pub (crate) sub_suggestion : Option < UnreachableLabelSubSuggestion > , # [subdiagnostic] pub (crate) sub_suggestion_label : Option < UnreachableLabelSubLabel > , # [subdiagnostic] pub (crate) sub_unreachable_label : Option < UnreachableLabelSubLabelUnreachable > , }
/* FP:errors.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0046
/* FP:errors.rs-0092 */ # [derive (Subdiagnostic)] # [suggestion (resolve_unreachable_label_suggestion_use_similarly_named , code = "{ident_name}" , applicability = "maybe-incorrect")] pub (crate) struct UnreachableLabelSubSuggestion { # [primary_span] pub (crate) span : Span , pub (crate) ident_name : Symbol , }
/* FP:errors.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0047
/* FP:errors.rs-0094 */ # [derive (Subdiagnostic)] # [label (resolve_unreachable_label_similar_name_reachable)] pub (crate) struct UnreachableLabelSubLabel { # [primary_span] pub (crate) ident_span : Span , }
/* FP:errors.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0048
/* FP:errors.rs-0096 */ # [derive (Subdiagnostic)] # [label (resolve_unreachable_label_similar_name_unreachable)] pub (crate) struct UnreachableLabelSubLabelUnreachable { # [primary_span] pub (crate) ident_span : Span , }
/* FP:errors.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0049
/* FP:errors.rs-0098 */ # [derive (Diagnostic)] # [diag (resolve_invalid_asm_sym)] # [help] pub (crate) struct InvalidAsmSym { # [primary_span] # [label] pub (crate) span : Span , }
/* FP:errors.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0050
/* FP:errors.rs-0100 */ # [derive (Diagnostic)] # [diag (resolve_lowercase_self)] pub (crate) struct LowercaseSelf { # [primary_span] # [suggestion (code = "Self" , applicability = "maybe-incorrect" , style = "short")] pub (crate) span : Span , }
/* FP:errors.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0051
/* FP:errors.rs-0102 */ # [derive (Debug)] # [derive (Diagnostic)] # [diag (resolve_binding_in_never_pattern)] pub (crate) struct BindingInNeverPattern { # [primary_span] # [suggestion (code = "_" , applicability = "machine-applicable" , style = "short")] pub (crate) span : Span , }
/* FP:errors.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0052
/* FP:errors.rs-0104 */ # [derive (Diagnostic)] # [diag (resolve_trait_impl_duplicate , code = E0201)] pub (crate) struct TraitImplDuplicate { # [primary_span] # [label] pub (crate) span : Span , # [label (resolve_old_span_label)] pub (crate) old_span : Span , # [label (resolve_trait_item_span)] pub (crate) trait_item_span : Span , pub (crate) name : Ident , }
/* FP:errors.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0053
/* FP:errors.rs-0106 */ # [derive (Diagnostic)] # [diag (resolve_relative_2018)] pub (crate) struct Relative2018 { # [primary_span] pub (crate) span : Span , # [suggestion (code = "crate::{path_str}" , applicability = "maybe-incorrect")] pub (crate) path_span : Span , pub (crate) path_str : String , }
/* FP:errors.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0054
/* FP:errors.rs-0108 */ # [derive (Diagnostic)] # [diag (resolve_ancestor_only , code = E0742)] pub (crate) struct AncestorOnly (# [primary_span] pub (crate) Span) ;
/* FP:errors.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0055
/* FP:errors.rs-0110 */ # [derive (Diagnostic)] # [diag (resolve_expected_module_found , code = E0577)] pub (crate) struct ExpectedModuleFound { # [primary_span] # [label] pub (crate) span : Span , pub (crate) res : Res , pub (crate) path_str : String , }
/* FP:errors.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0056
/* FP:errors.rs-0112 */ # [derive (Diagnostic)] # [diag (resolve_indeterminate , code = E0578)] pub (crate) struct Indeterminate (# [primary_span] pub (crate) Span) ;
/* FP:errors.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0057
/* FP:errors.rs-0114 */ # [derive (Diagnostic)] # [diag (resolve_tool_module_imported)] pub (crate) struct ToolModuleImported { # [primary_span] pub (crate) span : Span , # [note] pub (crate) import : Span , }
/* FP:errors.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0058
/* FP:errors.rs-0116 */ # [derive (Diagnostic)] # [diag (resolve_module_only)] pub (crate) struct ModuleOnly (# [primary_span] pub (crate) Span) ;
/* FP:errors.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0059
/* FP:errors.rs-0118 */ # [derive (Diagnostic)] # [diag (resolve_macro_expected_found)] pub (crate) struct MacroExpectedFound < 'a > { # [primary_span] # [label] pub (crate) span : Span , pub (crate) found : & 'a str , pub (crate) article : & 'static str , pub (crate) expected : & 'a str , pub (crate) macro_path : & 'a str , # [subdiagnostic] pub (crate) remove_surrounding_derive : Option < RemoveSurroundingDerive > , # [subdiagnostic] pub (crate) add_as_non_derive : Option < AddAsNonDerive < 'a > > , }
/* FP:errors.rs-0119 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0060
/* FP:errors.rs-0120 */ # [derive (Subdiagnostic)] # [help (resolve_remove_surrounding_derive)] pub (crate) struct RemoveSurroundingDerive { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0121 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0061
/* FP:errors.rs-0122 */ # [derive (Subdiagnostic)] # [help (resolve_add_as_non_derive)] pub (crate) struct AddAsNonDerive < 'a > { pub (crate) macro_path : & 'a str , }
/* FP:errors.rs-0123 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0062
/* FP:errors.rs-0124 */ # [derive (Diagnostic)] # [diag (resolve_proc_macro_same_crate)] pub (crate) struct ProcMacroSameCrate { # [primary_span] pub (crate) span : Span , # [help] pub (crate) is_test : bool , }
/* FP:errors.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0063
/* FP:errors.rs-0126 */ # [derive (Diagnostic)] # [diag (resolve_imported_crate)] pub (crate) struct CrateImported { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0127 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0064
/* FP:errors.rs-0128 */ # [derive (Diagnostic)] # [diag (resolve_macro_use_extern_crate_self)] pub (crate) struct MacroUseExternCrateSelf { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0129 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0065
/* FP:errors.rs-0130 */ # [derive (Diagnostic)] # [diag (resolve_accessible_unsure)] # [note] pub (crate) struct CfgAccessibleUnsure { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0066
/* FP:errors.rs-0132 */ # [derive (Debug)] # [derive (Diagnostic)] # [diag (resolve_param_in_enum_discriminant)] pub (crate) struct ParamInEnumDiscriminant { # [primary_span] # [label] pub (crate) span : Span , pub (crate) name : Symbol , # [subdiagnostic] pub (crate) param_kind : ParamKindInEnumDiscriminant , }
/* FP:errors.rs-0133 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_ENUM_0067
/* FP:errors.rs-0134 */ # [derive (Debug)] # [derive (Subdiagnostic)] pub (crate) enum ParamKindInEnumDiscriminant { # [note (resolve_type_param_in_enum_discriminant)] Type , # [note (resolve_const_param_in_enum_discriminant)] Const , # [note (resolve_lifetime_param_in_enum_discriminant)] Lifetime , }
/* FP:errors.rs-0135 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0068
/* FP:errors.rs-0136 */ # [derive (Subdiagnostic)] # [label (resolve_change_import_binding)] pub (crate) struct ChangeImportBinding { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0137 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0069
/* FP:errors.rs-0138 */ # [derive (Subdiagnostic)] # [suggestion (resolve_change_import_binding , code = "{suggestion}" , applicability = "maybe-incorrect")] pub (crate) struct ChangeImportBindingSuggestion { # [primary_span] pub (crate) span : Span , pub (crate) suggestion : String , }
/* FP:errors.rs-0139 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0070
/* FP:errors.rs-0140 */ # [derive (Diagnostic)] # [diag (resolve_imports_cannot_refer_to)] pub (crate) struct ImportsCannotReferTo < 'a > { # [primary_span] pub (crate) span : Span , pub (crate) what : & 'a str , }
/* FP:errors.rs-0141 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0071
/* FP:errors.rs-0142 */ # [derive (Diagnostic)] # [diag (resolve_cannot_find_ident_in_this_scope)] pub (crate) struct CannotFindIdentInThisScope < 'a > { # [primary_span] pub (crate) span : Span , pub (crate) expected : & 'a str , pub (crate) ident : Ident , }
/* FP:errors.rs-0143 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0072
/* FP:errors.rs-0144 */ # [derive (Subdiagnostic)] # [note (resolve_explicit_unsafe_traits)] pub (crate) struct ExplicitUnsafeTraits { # [primary_span] pub (crate) span : Span , pub (crate) ident : Ident , }
/* FP:errors.rs-0145 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0073
/* FP:errors.rs-0146 */ # [derive (Subdiagnostic)] # [note (resolve_macro_defined_later)] pub (crate) struct MacroDefinedLater { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0147 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0074
/* FP:errors.rs-0148 */ # [derive (Subdiagnostic)] # [label (resolve_consider_move_macro_position)] pub (crate) struct MacroSuggMovePosition { # [primary_span] pub (crate) span : Span , pub (crate) ident : Ident , }
/* FP:errors.rs-0149 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_ENUM_0075
/* FP:errors.rs-0150 */ # [derive (Subdiagnostic)] pub (crate) enum MacroRulesNot { # [label (resolve_macro_cannot_use_as_fn_like)] Func { # [primary_span] span : Span , ident : Ident , } , # [label (resolve_macro_cannot_use_as_attr)] Attr { # [primary_span] span : Span , ident : Ident , } , # [label (resolve_macro_cannot_use_as_derive)] Derive { # [primary_span] span : Span , ident : Ident , } , }
/* FP:errors.rs-0151 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0076
/* FP:errors.rs-0152 */ # [derive (Subdiagnostic)] # [note (resolve_missing_macro_rules_name)] pub (crate) struct MaybeMissingMacroRulesName { # [primary_span] pub (crate) spans : MultiSpan , }
/* FP:errors.rs-0153 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0077
/* FP:errors.rs-0154 */ # [derive (Subdiagnostic)] # [help (resolve_added_macro_use)] pub (crate) struct AddedMacroUse ;
/* FP:errors.rs-0155 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0078
/* FP:errors.rs-0156 */ # [derive (Subdiagnostic)] # [suggestion (resolve_consider_adding_a_derive , code = "{suggestion}" , applicability = "maybe-incorrect")] pub (crate) struct ConsiderAddingADerive { # [primary_span] pub (crate) span : Span , pub (crate) suggestion : String , }
/* FP:errors.rs-0157 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0079
/* FP:errors.rs-0158 */ # [derive (Diagnostic)] # [diag (resolve_cannot_determine_import_resolution)] pub (crate) struct CannotDetermineImportResolution { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0159 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0080
/* FP:errors.rs-0160 */ # [derive (Diagnostic)] # [diag (resolve_cannot_determine_macro_resolution)] # [note] pub (crate) struct CannotDetermineMacroResolution { # [primary_span] pub (crate) span : Span , pub (crate) kind : & 'static str , pub (crate) path : String , }
/* FP:errors.rs-0161 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0081
/* FP:errors.rs-0162 */ # [derive (Diagnostic)] # [diag (resolve_cannot_be_reexported_private , code = E0364)] pub (crate) struct CannotBeReexportedPrivate { # [primary_span] pub (crate) span : Span , pub (crate) ident : Ident , }
/* FP:errors.rs-0163 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0082
/* FP:errors.rs-0164 */ # [derive (Diagnostic)] # [diag (resolve_cannot_be_reexported_crate_public , code = E0364)] pub (crate) struct CannotBeReexportedCratePublic { # [primary_span] pub (crate) span : Span , pub (crate) ident : Ident , }
/* FP:errors.rs-0165 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0083
/* FP:errors.rs-0166 */ # [derive (Diagnostic)] # [diag (resolve_cannot_be_reexported_private , code = E0365)] # [note (resolve_consider_declaring_with_pub)] pub (crate) struct CannotBeReexportedPrivateNS { # [primary_span] # [label (resolve_reexport_of_private)] pub (crate) span : Span , pub (crate) ident : Ident , }
/* FP:errors.rs-0167 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0084
/* FP:errors.rs-0168 */ # [derive (Diagnostic)] # [diag (resolve_cannot_be_reexported_crate_public , code = E0365)] # [note (resolve_consider_declaring_with_pub)] pub (crate) struct CannotBeReexportedCratePublicNS { # [primary_span] # [label (resolve_reexport_of_crate_public)] pub (crate) span : Span , pub (crate) ident : Ident , }
/* FP:errors.rs-0169 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0085
/* FP:errors.rs-0170 */ # [derive (Subdiagnostic)] # [help (resolve_consider_adding_macro_export)] pub (crate) struct ConsiderAddingMacroExport { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0171 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0086
/* FP:errors.rs-0172 */ # [derive (Subdiagnostic)] # [suggestion (resolve_consider_marking_as_pub_crate , code = "pub(crate)" , applicability = "maybe-incorrect")] pub (crate) struct ConsiderMarkingAsPubCrate { # [primary_span] pub (crate) vis_span : Span , }
/* FP:errors.rs-0173 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0087
/* FP:errors.rs-0174 */ # [derive (Subdiagnostic)] # [note (resolve_consider_marking_as_pub)] pub (crate) struct ConsiderMarkingAsPub { # [primary_span] pub (crate) span : Span , pub (crate) ident : Ident , }
/* FP:errors.rs-0175 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0088
/* FP:errors.rs-0176 */ # [derive (Diagnostic)] # [diag (resolve_cannot_glob_import_possible_crates)] pub (crate) struct CannotGlobImportAllCrates { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0177 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0089
/* FP:errors.rs-0178 */ # [derive (Subdiagnostic)] # [suggestion (resolve_unexpected_res_change_ty_to_const_param_sugg , code = "const " , style = "verbose")] pub (crate) struct UnexpectedResChangeTyToConstParamSugg { # [primary_span] pub span : Span , # [applicability] pub applicability : Applicability , }
/* FP:errors.rs-0179 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0090
/* FP:errors.rs-0180 */ # [derive (Subdiagnostic)] # [suggestion (resolve_unexpected_res_use_at_op_in_slice_pat_with_range_sugg , code = "{snippet}" , applicability = "maybe-incorrect" , style = "verbose")] pub (crate) struct UnexpectedResUseAtOpInSlicePatWithRangeSugg { # [primary_span] pub span : Span , pub ident : Ident , pub snippet : String , }
/* FP:errors.rs-0181 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0091
/* FP:errors.rs-0182 */ # [derive (Diagnostic)] # [diag (resolve_extern_crate_loading_macro_not_at_crate_root , code = E0468)] pub (crate) struct ExternCrateLoadingMacroNotAtCrateRoot { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0183 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0092
/* FP:errors.rs-0184 */ # [derive (Diagnostic)] # [diag (resolve_extern_crate_self_requires_renaming)] pub (crate) struct ExternCrateSelfRequiresRenaming { # [primary_span] # [suggestion (code = "extern crate self as name;" , applicability = "has-placeholders")] pub (crate) span : Span , }
/* FP:errors.rs-0185 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0093
/* FP:errors.rs-0186 */ # [derive (Diagnostic)] # [diag (resolve_macro_use_name_already_in_use)] # [note] pub (crate) struct MacroUseNameAlreadyInUse { # [primary_span] pub (crate) span : Span , pub (crate) name : Symbol , }
/* FP:errors.rs-0187 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0094
/* FP:errors.rs-0188 */ # [derive (Diagnostic)] # [diag (resolve_imported_macro_not_found , code = E0469)] pub (crate) struct ImportedMacroNotFound { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0189 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0095
/* FP:errors.rs-0190 */ # [derive (Diagnostic)] # [diag (resolve_macro_extern_deprecated)] pub (crate) struct MacroExternDeprecated { # [primary_span] pub (crate) span : Span , # [help] pub inner_attribute : bool , }
/* FP:errors.rs-0191 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0096
/* FP:errors.rs-0192 */ # [derive (Diagnostic)] # [diag (resolve_arguments_macro_use_not_allowed)] pub (crate) struct ArgumentsMacroUseNotAllowed { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0193 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0097
/* FP:errors.rs-0194 */ # [derive (Diagnostic)] # [diag (resolve_unnamed_crate_root_import)] pub (crate) struct UnnamedCrateRootImport { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0195 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0098
/* FP:errors.rs-0196 */ # [derive (Diagnostic)] # [diag (resolve_macro_expanded_extern_crate_cannot_shadow_extern_arguments)] pub (crate) struct MacroExpandedExternCrateCannotShadowExternArguments { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0197 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0099
/* FP:errors.rs-0198 */ # [derive (Diagnostic)] # [diag (resolve_elided_anonymous_lifetime_report_error , code = E0637)] pub (crate) struct ElidedAnonymousLifetimeReportError { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) suggestion : Option < ElidedAnonymousLifetimeReportErrorSuggestion > , }
/* FP:errors.rs-0199 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0100
/* FP:errors.rs-0200 */ # [derive (Diagnostic)] # [diag (resolve_lending_iterator_report_error)] pub (crate) struct LendingIteratorReportError { # [primary_span] pub (crate) lifetime : Span , # [note] pub (crate) ty : Span , }
/* FP:errors.rs-0201 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0101
/* FP:errors.rs-0202 */ # [derive (Diagnostic)] # [diag (resolve_anonymous_lifetime_non_gat_report_error)] pub (crate) struct AnonymousLifetimeNonGatReportError { # [primary_span] # [label] pub (crate) lifetime : Span , }
/* FP:errors.rs-0203 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0102
/* FP:errors.rs-0204 */ # [derive (Subdiagnostic)] # [multipart_suggestion (resolve_elided_anonymous_lifetime_report_error_suggestion , applicability = "machine-applicable")] pub (crate) struct ElidedAnonymousLifetimeReportErrorSuggestion { # [suggestion_part (code = "for<'a> ")] pub (crate) lo : Span , # [suggestion_part (code = "'a ")] pub (crate) hi : Span , }
/* FP:errors.rs-0205 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0103
/* FP:errors.rs-0206 */ # [derive (Diagnostic)] # [diag (resolve_explicit_anonymous_lifetime_report_error , code = E0637)] pub (crate) struct ExplicitAnonymousLifetimeReportError { # [primary_span] # [label] pub (crate) span : Span , }
/* FP:errors.rs-0207 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0104
/* FP:errors.rs-0208 */ # [derive (Diagnostic)] # [diag (resolve_implicit_elided_lifetimes_not_allowed_here , code = E0726)] pub (crate) struct ImplicitElidedLifetimeNotAllowedHere { # [primary_span] pub (crate) span : Span , # [subdiagnostic] pub (crate) subdiag : ElidedLifetimeInPathSubdiag , }
/* FP:errors.rs-0209 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0105
/* FP:errors.rs-0210 */ # [derive (Diagnostic)] # [diag (resolve_underscore_lifetime_is_reserved , code = E0637)] # [help] pub (crate) struct UnderscoreLifetimeIsReserved { # [primary_span] # [label] pub (crate) span : Span , }
/* FP:errors.rs-0211 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0106
/* FP:errors.rs-0212 */ # [derive (Diagnostic)] # [diag (resolve_static_lifetime_is_reserved , code = E0262)] pub (crate) struct StaticLifetimeIsReserved { # [primary_span] # [label] pub (crate) span : Span , pub (crate) lifetime : Ident , }
/* FP:errors.rs-0213 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0107
/* FP:errors.rs-0214 */ # [derive (Diagnostic)] # [diag (resolve_variable_is_not_bound_in_all_patterns , code = E0408)] pub (crate) struct VariableIsNotBoundInAllPatterns { # [primary_span] pub (crate) multispan : MultiSpan , pub (crate) name : Ident , }
/* FP:errors.rs-0215 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0108
/* FP:errors.rs-0216 */ # [derive (Subdiagnostic , Debug , Clone)] # [label (resolve_pattern_doesnt_bind_name)] pub (crate) struct PatternDoesntBindName { # [primary_span] pub (crate) span : Span , pub (crate) name : Ident , }
/* FP:errors.rs-0217 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0109
/* FP:errors.rs-0218 */ # [derive (Subdiagnostic , Debug , Clone)] # [label (resolve_variable_not_in_all_patterns)] pub (crate) struct VariableNotInAllPatterns { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0219 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0110
/* FP:errors.rs-0220 */ # [derive (Subdiagnostic)] # [multipart_suggestion (resolve_variable_is_a_typo , applicability = "maybe-incorrect" , style = "verbose")] pub (crate) struct PatternBindingTypo { # [suggestion_part (code = "{typo}")] pub (crate) spans : Vec < Span > , pub (crate) typo : Symbol , }
/* FP:errors.rs-0221 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0111
/* FP:errors.rs-0222 */ # [derive (Diagnostic)] # [diag (resolve_name_defined_multiple_time)] # [note] pub (crate) struct NameDefinedMultipleTime { # [primary_span] pub (crate) span : Span , pub (crate) name : Symbol , pub (crate) descr : & 'static str , pub (crate) container : & 'static str , # [subdiagnostic] pub (crate) label : NameDefinedMultipleTimeLabel , # [subdiagnostic] pub (crate) old_binding_label : Option < NameDefinedMultipleTimeOldBindingLabel > , }
/* FP:errors.rs-0223 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_ENUM_0112
/* FP:errors.rs-0224 */ # [derive (Subdiagnostic)] pub (crate) enum NameDefinedMultipleTimeLabel { # [label (resolve_name_defined_multiple_time_reimported)] Reimported { # [primary_span] span : Span , } , # [label (resolve_name_defined_multiple_time_redefined)] Redefined { # [primary_span] span : Span , } , }
/* FP:errors.rs-0225 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_ENUM_0113
/* FP:errors.rs-0226 */ # [derive (Subdiagnostic)] pub (crate) enum NameDefinedMultipleTimeOldBindingLabel { # [label (resolve_name_defined_multiple_time_old_binding_import)] Import { # [primary_span] span : Span , old_kind : & 'static str , } , # [label (resolve_name_defined_multiple_time_old_binding_definition)] Definition { # [primary_span] span : Span , old_kind : & 'static str , } , }
/* FP:errors.rs-0227 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0114
/* FP:errors.rs-0228 */ # [derive (Diagnostic)] # [diag (resolve_is_private , code = E0603)] pub (crate) struct IsPrivate < 'a > { # [primary_span] # [label] pub (crate) span : Span , pub (crate) ident_descr : & 'a str , pub (crate) ident : Ident , }
/* FP:errors.rs-0229 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0115
/* FP:errors.rs-0230 */ # [derive (Diagnostic)] # [diag (resolve_generic_arguments_in_macro_path)] pub (crate) struct GenericArgumentsInMacroPath { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0231 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0116
/* FP:errors.rs-0232 */ # [derive (Diagnostic)] # [diag (resolve_attributes_starting_with_rustc_are_reserved)] pub (crate) struct AttributesStartingWithRustcAreReserved { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0233 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0117
/* FP:errors.rs-0234 */ # [derive (Diagnostic)] # [diag (resolve_cannot_use_through_an_import)] pub (crate) struct CannotUseThroughAnImport { # [primary_span] pub (crate) span : Span , pub (crate) article : & 'static str , pub (crate) descr : & 'static str , # [note] pub (crate) binding_span : Option < Span > , }
/* FP:errors.rs-0235 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0118
/* FP:errors.rs-0236 */ # [derive (Diagnostic)] # [diag (resolve_name_reserved_in_attribute_namespace)] pub (crate) struct NameReservedInAttributeNamespace { # [primary_span] pub (crate) span : Span , pub (crate) ident : Ident , }
/* FP:errors.rs-0237 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0119
/* FP:errors.rs-0238 */ # [derive (Diagnostic)] # [diag (resolve_cannot_find_builtin_macro_with_name)] pub (crate) struct CannotFindBuiltinMacroWithName { # [primary_span] pub (crate) span : Span , pub (crate) ident : Ident , }
/* FP:errors.rs-0239 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0120
/* FP:errors.rs-0240 */ # [derive (Diagnostic)] # [diag (resolve_tool_was_already_registered)] pub (crate) struct ToolWasAlreadyRegistered { # [primary_span] pub (crate) span : Span , pub (crate) tool : Ident , # [label] pub (crate) old_ident_span : Span , }
/* FP:errors.rs-0241 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0121
/* FP:errors.rs-0242 */ # [derive (Diagnostic)] # [diag (resolve_tool_only_accepts_identifiers)] pub (crate) struct ToolOnlyAcceptsIdentifiers { # [primary_span] # [label] pub (crate) span : Span , pub (crate) tool : Symbol , }
/* FP:errors.rs-0243 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_ENUM_0122
/* FP:errors.rs-0244 */ # [derive (Subdiagnostic)] pub (crate) enum DefinedHere { # [label (resolve_similarly_named_defined_here)] SimilarlyNamed { # [primary_span] span : Span , candidate_descr : & 'static str , candidate : Symbol , } , # [label (resolve_single_item_defined_here)] SingleItem { # [primary_span] span : Span , candidate_descr : & 'static str , candidate : Symbol , } , }
/* FP:errors.rs-0245 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0123
/* FP:errors.rs-0246 */ # [derive (Subdiagnostic)] # [label (resolve_outer_ident_is_not_publicly_reexported)] pub (crate) struct OuterIdentIsNotPubliclyReexported { # [primary_span] pub (crate) span : Span , pub (crate) outer_ident_descr : & 'static str , pub (crate) outer_ident : Ident , }
/* FP:errors.rs-0247 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0124
/* FP:errors.rs-0248 */ # [derive (Subdiagnostic)] # [label (resolve_constructor_private_if_any_field_private)] pub (crate) struct ConstructorPrivateIfAnyFieldPrivate { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0249 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0125
/* FP:errors.rs-0250 */ # [derive (Subdiagnostic)] # [multipart_suggestion (resolve_consider_making_the_field_public , applicability = "maybe-incorrect" , style = "verbose")] pub (crate) struct ConsiderMakingTheFieldPublic { # [suggestion_part (code = "pub ")] pub (crate) spans : Vec < Span > , pub (crate) number_of_fields : usize , }
/* FP:errors.rs-0251 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_ENUM_0126
/* FP:errors.rs-0252 */ # [derive (Subdiagnostic)] pub (crate) enum ImportIdent { # [suggestion (resolve_suggestion_import_ident_through_reexport , code = "{path}" , applicability = "machine-applicable" , style = "verbose")] ThroughReExport { # [primary_span] span : Span , ident : Ident , path : String , } , # [suggestion (resolve_suggestion_import_ident_directly , code = "{path}" , applicability = "machine-applicable" , style = "verbose")] Directly { # [primary_span] span : Span , ident : Ident , path : String , } , }
/* FP:errors.rs-0253 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0127
/* FP:errors.rs-0254 */ # [derive (Subdiagnostic)] # [note (resolve_note_and_refers_to_the_item_defined_here)] pub (crate) struct NoteAndRefersToTheItemDefinedHere < 'a > { # [primary_span] pub (crate) span : MultiSpan , pub (crate) binding_descr : & 'a str , pub (crate) binding_name : Ident , pub (crate) first : bool , pub (crate) dots : bool , }
/* FP:errors.rs-0255 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0128
/* FP:errors.rs-0256 */ # [derive (Subdiagnostic)] # [suggestion (resolve_remove_unnecessary_import , code = "" , applicability = "maybe-incorrect")] pub (crate) struct RemoveUnnecessaryImport { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0257 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0129
/* FP:errors.rs-0258 */ # [derive (Subdiagnostic)] # [suggestion (resolve_remove_unnecessary_import , code = "" , applicability = "maybe-incorrect" , style = "tool-only")] pub (crate) struct ToolOnlyRemoveUnnecessaryImport { # [primary_span] pub (crate) span : Span , }
/* FP:errors.rs-0259 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0130
/* FP:errors.rs-0260 */ # [derive (Subdiagnostic)] # [note (resolve_ident_imported_here_but_it_is_desc)] pub (crate) struct IdentImporterHereButItIsDesc < 'a > { # [primary_span] pub (crate) span : Span , pub (crate) imported_ident : Ident , pub (crate) imported_ident_desc : & 'a str , }
/* FP:errors.rs-0261 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0131
/* FP:errors.rs-0262 */ # [derive (Subdiagnostic)] # [note (resolve_ident_in_scope_but_it_is_desc)] pub (crate) struct IdentInScopeButItIsDesc < 'a > { pub (crate) imported_ident : Ident , pub (crate) imported_ident_desc : & 'a str , }
/* FP:errors.rs-0263 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0132
/* FP:errors.rs-0264 */ pub (crate) struct FoundItemConfigureOut { pub (crate) span : Span , pub (crate) item_was : ItemWas , }
/* FP:errors.rs-0265 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_ENUM_0133
/* FP:errors.rs-0266 */ pub (crate) enum ItemWas { BehindFeature { feature : Symbol , span : Span } , CfgOut { span : Span } , }
/* FP:errors.rs-0267 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_IMPL_0134
/* FP:errors.rs-0268 */ impl Subdiagnostic for FoundItemConfigureOut { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { let mut multispan : MultiSpan = self . span . into () ; match self . item_was { ItemWas :: BehindFeature { feature , span } => { let key = "feature" . into () ; let value = feature . into_diag_arg (& mut None) ; let msg = diag . dcx . eagerly_translate_to_string (fluent :: resolve_item_was_behind_feature , [(& key , & value)] . into_iter () ,) ; multispan . push_span_label (span , msg) ; } ItemWas :: CfgOut { span } => { multispan . push_span_label (span , fluent :: resolve_item_was_cfg_out) ; } } diag . span_note (multispan , fluent :: resolve_found_an_item_configured_out) ; } }
/* FP:errors.rs-0269 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_errors_STRUCT_0135
/* FP:errors.rs-0270 */ # [derive (Diagnostic)] # [diag (resolve_trait_impl_mismatch)] pub (crate) struct TraitImplMismatch { # [primary_span] # [label] pub (crate) span : Span , pub (crate) name : Ident , pub (crate) kind : & 'static str , pub (crate) trait_path : String , # [label (resolve_trait_impl_mismatch_label_item)] pub (crate) trait_item_span : Span , }