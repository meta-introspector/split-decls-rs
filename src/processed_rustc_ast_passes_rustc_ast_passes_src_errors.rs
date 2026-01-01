/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_USE_0001
/* FP:errors.rs-0002 */ use crate :: rustc_abi :: ExternAbi ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_USE_0002
/* FP:errors.rs-0004 */ use crate :: rustc_complete :: ParamKindOrd ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_USE_0003
/* FP:errors.rs-0006 */ use crate :: rustc_complete :: codes :: * ;
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_USE_0004
/* FP:errors.rs-0008 */ use crate :: rustc_complete :: { Applicability , Diag , EmissionGuarantee , Subdiagnostic } ;
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_USE_0005
/* FP:errors.rs-0010 */ use rustc_macros :: { Diagnostic , LintDiagnostic , Subdiagnostic } ;
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_USE_0006
/* FP:errors.rs-0012 */ use crate :: rustc_complete :: { Ident , Span , Symbol } ;
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_USE_0007
/* FP:errors.rs-0014 */ use crate :: fluent_generated as fluent ;
/* FP:errors.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0008
/* FP:errors.rs-0016 */ # [derive (Diagnostic)] # [diag (ast_passes_visibility_not_permitted , code = E0449)] pub (crate) struct VisibilityNotPermitted { # [primary_span] pub span : Span , # [subdiagnostic] pub note : VisibilityNotPermittedNote , # [suggestion (ast_passes_remove_qualifier_sugg , code = "" , applicability = "machine-applicable")] pub remove_qualifier_sugg : Span , }
/* FP:errors.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_ENUM_0009
/* FP:errors.rs-0018 */ # [derive (Subdiagnostic)] pub (crate) enum VisibilityNotPermittedNote { # [note (ast_passes_enum_variant)] EnumVariant , # [note (ast_passes_trait_impl)] TraitImpl , # [note (ast_passes_individual_impl_items)] IndividualImplItems , # [note (ast_passes_individual_foreign_items)] IndividualForeignItems , }
/* FP:errors.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0010
/* FP:errors.rs-0020 */ # [derive (Diagnostic)] # [diag (ast_passes_trait_fn_const , code = E0379)] pub (crate) struct TraitFnConst { # [primary_span] # [label] pub span : Span , pub in_impl : bool , # [label (ast_passes_const_context_label)] pub const_context_label : Option < Span > , # [suggestion (ast_passes_remove_const_sugg , code = "")] pub remove_const_sugg : (Span , Applicability) , pub requires_multiple_changes : bool , # [suggestion (ast_passes_make_impl_const_sugg , code = "const " , applicability = "maybe-incorrect")] pub make_impl_const_sugg : Option < Span > , # [suggestion (ast_passes_make_trait_const_sugg , code = "#[const_trait]\n" , applicability = "maybe-incorrect")] pub make_trait_const_sugg : Option < Span > , }
/* FP:errors.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0011
/* FP:errors.rs-0022 */ # [derive (Diagnostic)] # [diag (ast_passes_async_fn_in_const_trait_or_trait_impl)] pub (crate) struct AsyncFnInConstTraitOrTraitImpl { # [primary_span] pub async_keyword : Span , pub in_impl : bool , # [label] pub const_keyword : Span , }
/* FP:errors.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0012
/* FP:errors.rs-0024 */ # [derive (Diagnostic)] # [diag (ast_passes_forbidden_bound)] pub (crate) struct ForbiddenBound { # [primary_span] pub spans : Vec < Span > , }
/* FP:errors.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0013
/* FP:errors.rs-0026 */ # [derive (Diagnostic)] # [diag (ast_passes_forbidden_const_param)] pub (crate) struct ForbiddenConstParam { # [primary_span] pub const_param_spans : Vec < Span > , }
/* FP:errors.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0014
/* FP:errors.rs-0028 */ # [derive (Diagnostic)] # [diag (ast_passes_fn_param_too_many)] pub (crate) struct FnParamTooMany { # [primary_span] pub span : Span , pub max_num_args : usize , }
/* FP:errors.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0015
/* FP:errors.rs-0030 */ # [derive (Diagnostic)] # [diag (ast_passes_fn_param_c_var_args_not_last)] pub (crate) struct FnParamCVarArgsNotLast { # [primary_span] pub span : Span , }
/* FP:errors.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0016
/* FP:errors.rs-0032 */ # [derive (Diagnostic)] # [diag (ast_passes_fn_param_doc_comment)] pub (crate) struct FnParamDocComment { # [primary_span] # [label] pub span : Span , }
/* FP:errors.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0017
/* FP:errors.rs-0034 */ # [derive (Diagnostic)] # [diag (ast_passes_fn_param_forbidden_attr)] pub (crate) struct FnParamForbiddenAttr { # [primary_span] pub span : Span , }
/* FP:errors.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0018
/* FP:errors.rs-0036 */ # [derive (Diagnostic)] # [diag (ast_passes_fn_param_forbidden_self)] # [note] pub (crate) struct FnParamForbiddenSelf { # [primary_span] # [label] pub span : Span , }
/* FP:errors.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0019
/* FP:errors.rs-0038 */ # [derive (Diagnostic)] # [diag (ast_passes_forbidden_default)] pub (crate) struct ForbiddenDefault { # [primary_span] pub span : Span , # [label] pub def_span : Span , }
/* FP:errors.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0020
/* FP:errors.rs-0040 */ # [derive (Diagnostic)] # [diag (ast_passes_assoc_const_without_body)] pub (crate) struct AssocConstWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " = <expr>;" , applicability = "has-placeholders")] pub replace_span : Span , }
/* FP:errors.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0021
/* FP:errors.rs-0042 */ # [derive (Diagnostic)] # [diag (ast_passes_assoc_fn_without_body)] pub (crate) struct AssocFnWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " {{ <body> }}" , applicability = "has-placeholders")] pub replace_span : Span , }
/* FP:errors.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0022
/* FP:errors.rs-0044 */ # [derive (Diagnostic)] # [diag (ast_passes_assoc_type_without_body)] pub (crate) struct AssocTypeWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " = <type>;" , applicability = "has-placeholders")] pub replace_span : Span , }
/* FP:errors.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0023
/* FP:errors.rs-0046 */ # [derive (Diagnostic)] # [diag (ast_passes_const_without_body)] pub (crate) struct ConstWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " = <expr>;" , applicability = "has-placeholders")] pub replace_span : Span , }
/* FP:errors.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0024
/* FP:errors.rs-0048 */ # [derive (Diagnostic)] # [diag (ast_passes_static_without_body)] pub (crate) struct StaticWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " = <expr>;" , applicability = "has-placeholders")] pub replace_span : Span , }
/* FP:errors.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0025
/* FP:errors.rs-0050 */ # [derive (Diagnostic)] # [diag (ast_passes_ty_alias_without_body)] pub (crate) struct TyAliasWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " = <type>;" , applicability = "has-placeholders")] pub replace_span : Span , }
/* FP:errors.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0026
/* FP:errors.rs-0052 */ # [derive (Diagnostic)] # [diag (ast_passes_fn_without_body)] pub (crate) struct FnWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " {{ <body> }}" , applicability = "has-placeholders")] pub replace_span : Span , # [subdiagnostic] pub extern_block_suggestion : Option < ExternBlockSuggestion > , }
/* FP:errors.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_ENUM_0027
/* FP:errors.rs-0054 */ # [derive (Subdiagnostic)] pub (crate) enum ExternBlockSuggestion { # [multipart_suggestion (ast_passes_extern_block_suggestion , applicability = "maybe-incorrect")] Implicit { # [suggestion_part (code = "extern {{")] start_span : Span , # [suggestion_part (code = " }}")] end_span : Span , } , # [multipart_suggestion (ast_passes_extern_block_suggestion , applicability = "maybe-incorrect")] Explicit { # [suggestion_part (code = "extern \"{abi}\" {{")] start_span : Span , # [suggestion_part (code = " }}")] end_span : Span , abi : Symbol , } , }
/* FP:errors.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0028
/* FP:errors.rs-0056 */ # [derive (Diagnostic)] # [diag (ast_passes_extern_invalid_safety)] pub (crate) struct InvalidSafetyOnExtern { # [primary_span] pub item_span : Span , # [suggestion (code = "unsafe " , applicability = "machine-applicable" , style = "verbose")] pub block : Option < Span > , }
/* FP:errors.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0029
/* FP:errors.rs-0058 */ # [derive (Diagnostic)] # [diag (ast_passes_item_invalid_safety)] pub (crate) struct InvalidSafetyOnItem { # [primary_span] pub span : Span , }
/* FP:errors.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0030
/* FP:errors.rs-0060 */ # [derive (Diagnostic)] # [diag (ast_passes_fn_ptr_invalid_safety)] pub (crate) struct InvalidSafetyOnFnPtr { # [primary_span] pub span : Span , }
/* FP:errors.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0031
/* FP:errors.rs-0062 */ # [derive (Diagnostic)] # [diag (ast_passes_unsafe_static)] pub (crate) struct UnsafeStatic { # [primary_span] pub span : Span , }
/* FP:errors.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0032
/* FP:errors.rs-0064 */ # [derive (Diagnostic)] # [diag (ast_passes_bound_in_context)] pub (crate) struct BoundInContext < 'a > { # [primary_span] pub span : Span , pub ctx : & 'a str , }
/* FP:errors.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0033
/* FP:errors.rs-0066 */ # [derive (Diagnostic)] # [diag (ast_passes_extern_types_cannot)] # [note (ast_passes_extern_keyword_link)] pub (crate) struct ExternTypesCannotHave < 'a > { # [primary_span] # [suggestion (code = "" , applicability = "maybe-incorrect")] pub span : Span , pub descr : & 'a str , pub remove_descr : & 'a str , # [label] pub block_span : Span , }
/* FP:errors.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0034
/* FP:errors.rs-0068 */ # [derive (Diagnostic)] # [diag (ast_passes_body_in_extern)] # [note (ast_passes_extern_keyword_link)] pub (crate) struct BodyInExtern < 'a > { # [primary_span] # [label (ast_passes_cannot_have)] pub span : Span , # [label (ast_passes_invalid)] pub body : Span , # [label (ast_passes_existing)] pub block : Span , pub kind : & 'a str , }
/* FP:errors.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0035
/* FP:errors.rs-0070 */ # [derive (Diagnostic)] # [diag (ast_passes_fn_body_extern)] # [help] # [note (ast_passes_extern_keyword_link)] pub (crate) struct FnBodyInExtern { # [primary_span] # [label (ast_passes_cannot_have)] pub span : Span , # [suggestion (code = ";" , applicability = "maybe-incorrect")] pub body : Span , # [label] pub block : Span , }
/* FP:errors.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0036
/* FP:errors.rs-0072 */ # [derive (Diagnostic)] # [diag (ast_passes_extern_fn_qualifiers)] pub (crate) struct FnQualifierInExtern { # [primary_span] # [suggestion (code = "" , applicability = "maybe-incorrect")] pub span : Span , # [label] pub block : Span , pub kw : & 'static str , }
/* FP:errors.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0037
/* FP:errors.rs-0074 */ # [derive (Diagnostic)] # [diag (ast_passes_extern_item_ascii)] # [note] pub (crate) struct ExternItemAscii { # [primary_span] pub span : Span , # [label] pub block : Span , }
/* FP:errors.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0038
/* FP:errors.rs-0076 */ # [derive (Diagnostic)] # [diag (ast_passes_c_variadic_associated_function)] pub (crate) struct CVariadicAssociatedFunction { # [primary_span] pub span : Span , }
/* FP:errors.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0039
/* FP:errors.rs-0078 */ # [derive (Diagnostic)] # [diag (ast_passes_c_variadic_no_extern)] # [help] pub (crate) struct CVariadicNoExtern { # [primary_span] pub span : Span , }
/* FP:errors.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0040
/* FP:errors.rs-0080 */ # [derive (Diagnostic)] # [diag (ast_passes_c_variadic_must_be_unsafe)] pub (crate) struct CVariadicMustBeUnsafe { # [primary_span] pub span : Span , # [suggestion (ast_passes_suggestion , applicability = "maybe-incorrect" , code = "unsafe " , style = "verbose")] pub unsafe_span : Span , }
/* FP:errors.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0041
/* FP:errors.rs-0082 */ # [derive (Diagnostic)] # [diag (ast_passes_c_variadic_bad_extern)] # [help] pub (crate) struct CVariadicBadExtern { # [primary_span] pub span : Span , pub abi : Symbol , # [label] pub extern_span : Span , }
/* FP:errors.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0042
/* FP:errors.rs-0084 */ # [derive (Diagnostic)] # [diag (ast_passes_item_underscore)] pub (crate) struct ItemUnderscore < 'a > { # [primary_span] # [label] pub span : Span , pub kind : & 'a str , }
/* FP:errors.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0043
/* FP:errors.rs-0086 */ # [derive (Diagnostic)] # [diag (ast_passes_nomangle_ascii , code = E0754)] pub (crate) struct NoMangleAscii { # [primary_span] pub span : Span , }
/* FP:errors.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0044
/* FP:errors.rs-0088 */ # [derive (Diagnostic)] # [diag (ast_passes_module_nonascii , code = E0754)] # [help] pub (crate) struct ModuleNonAscii { # [primary_span] pub span : Span , pub name : Symbol , }
/* FP:errors.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0045
/* FP:errors.rs-0090 */ # [derive (Diagnostic)] # [diag (ast_passes_auto_generic , code = E0567)] pub (crate) struct AutoTraitGeneric { # [primary_span] # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub span : Span , # [label] pub ident : Span , }
/* FP:errors.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0046
/* FP:errors.rs-0092 */ # [derive (Diagnostic)] # [diag (ast_passes_auto_super_lifetime , code = E0568)] pub (crate) struct AutoTraitBounds { # [primary_span] pub span : Vec < Span > , # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub removal : Span , # [label] pub ident : Span , }
/* FP:errors.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0047
/* FP:errors.rs-0094 */ # [derive (Diagnostic)] # [diag (ast_passes_auto_items , code = E0380)] pub (crate) struct AutoTraitItems { # [primary_span] pub spans : Vec < Span > , # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub total : Span , # [label] pub ident : Span , }
/* FP:errors.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0048
/* FP:errors.rs-0096 */ # [derive (Diagnostic)] # [diag (ast_passes_generic_before_constraints)] pub (crate) struct ArgsBeforeConstraint { # [primary_span] pub arg_spans : Vec < Span > , # [label (ast_passes_constraints)] pub constraints : Span , # [label (ast_passes_args)] pub args : Span , # [suggestion (code = "{suggestion}" , applicability = "machine-applicable" , style = "verbose")] pub data : Span , pub suggestion : String , pub constraint_len : usize , pub args_len : usize , # [subdiagnostic] pub constraint_spans : EmptyLabelManySpans , # [subdiagnostic] pub arg_spans2 : EmptyLabelManySpans , }
/* FP:errors.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0049
/* FP:errors.rs-0098 */ pub (crate) struct EmptyLabelManySpans (pub Vec < Span >) ;
/* FP:errors.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_IMPL_0050
/* FP:errors.rs-0100 */ impl Subdiagnostic for EmptyLabelManySpans { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . span_labels (self . 0 , "") ; } }
/* FP:errors.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0051
/* FP:errors.rs-0102 */ # [derive (Diagnostic)] # [diag (ast_passes_pattern_in_fn_pointer , code = E0561)] pub (crate) struct PatternFnPointer { # [primary_span] pub span : Span , }
/* FP:errors.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0052
/* FP:errors.rs-0104 */ # [derive (Diagnostic)] # [diag (ast_passes_trait_object_single_bound , code = E0226)] pub (crate) struct TraitObjectBound { # [primary_span] pub span : Span , }
/* FP:errors.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0053
/* FP:errors.rs-0106 */ # [derive (Diagnostic)] # [diag (ast_passes_nested_impl_trait , code = E0666)] pub (crate) struct NestedImplTrait { # [primary_span] pub span : Span , # [label (ast_passes_outer)] pub outer : Span , # [label (ast_passes_inner)] pub inner : Span , }
/* FP:errors.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0054
/* FP:errors.rs-0108 */ # [derive (Diagnostic)] # [diag (ast_passes_at_least_one_trait)] pub (crate) struct AtLeastOneTrait { # [primary_span] pub span : Span , }
/* FP:errors.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0055
/* FP:errors.rs-0110 */ # [derive (Diagnostic)] # [diag (ast_passes_out_of_order_params)] pub (crate) struct OutOfOrderParams < 'a > { # [primary_span] pub spans : Vec < Span > , # [suggestion (code = "{ordered_params}" , applicability = "machine-applicable")] pub sugg_span : Span , pub param_ord : & 'a ParamKindOrd , pub max_param : & 'a ParamKindOrd , pub ordered_params : & 'a str , }
/* FP:errors.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0056
/* FP:errors.rs-0112 */ # [derive (Diagnostic)] # [diag (ast_passes_obsolete_auto)] # [help] pub (crate) struct ObsoleteAuto { # [primary_span] pub span : Span , }
/* FP:errors.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0057
/* FP:errors.rs-0114 */ # [derive (Diagnostic)] # [diag (ast_passes_unsafe_negative_impl , code = E0198)] pub (crate) struct UnsafeNegativeImpl { # [primary_span] pub span : Span , # [label (ast_passes_negative)] pub negative : Span , # [label (ast_passes_unsafe)] pub r#unsafe : Span , }
/* FP:errors.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0058
/* FP:errors.rs-0116 */ # [derive (Diagnostic)] # [diag (ast_passes_unsafe_item)] pub (crate) struct UnsafeItem { # [primary_span] pub span : Span , pub kind : & 'static str , }
/* FP:errors.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0059
/* FP:errors.rs-0118 */ # [derive (Diagnostic)] # [diag (ast_passes_missing_unsafe_on_extern)] pub (crate) struct MissingUnsafeOnExtern { # [primary_span] pub span : Span , }
/* FP:errors.rs-0119 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0060
/* FP:errors.rs-0120 */ # [derive (Diagnostic)] # [diag (ast_passes_fieldless_union)] pub (crate) struct FieldlessUnion { # [primary_span] pub span : Span , }
/* FP:errors.rs-0121 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0061
/* FP:errors.rs-0122 */ # [derive (Diagnostic)] # [diag (ast_passes_where_clause_after_type_alias)] # [note] pub (crate) struct WhereClauseAfterTypeAlias { # [primary_span] pub span : Span , # [help] pub help : bool , }
/* FP:errors.rs-0123 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0062
/* FP:errors.rs-0124 */ # [derive (Diagnostic)] # [diag (ast_passes_where_clause_before_type_alias)] # [note] pub (crate) struct WhereClauseBeforeTypeAlias { # [primary_span] pub span : Span , # [subdiagnostic] pub sugg : WhereClauseBeforeTypeAliasSugg , }
/* FP:errors.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_ENUM_0063
/* FP:errors.rs-0126 */ # [derive (Subdiagnostic)] pub (crate) enum WhereClauseBeforeTypeAliasSugg { # [suggestion (ast_passes_remove_suggestion , applicability = "machine-applicable" , code = "")] Remove { # [primary_span] span : Span , } , # [multipart_suggestion (ast_passes_move_suggestion , applicability = "machine-applicable" , style = "verbose")] Move { # [suggestion_part (code = "")] left : Span , snippet : String , # [suggestion_part (code = "{snippet}")] right : Span , } , }
/* FP:errors.rs-0127 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0064
/* FP:errors.rs-0128 */ # [derive (Diagnostic)] # [diag (ast_passes_generic_default_trailing)] pub (crate) struct GenericDefaultTrailing { # [primary_span] pub span : Span , }
/* FP:errors.rs-0129 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0065
/* FP:errors.rs-0130 */ # [derive (Diagnostic)] # [diag (ast_passes_nested_lifetimes , code = E0316)] pub (crate) struct NestedLifetimes { # [primary_span] pub span : Span , }
/* FP:errors.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0066
/* FP:errors.rs-0132 */ # [derive (Diagnostic)] # [diag (ast_passes_const_bound_trait_object)] pub (crate) struct ConstBoundTraitObject { # [primary_span] pub span : Span , }
/* FP:errors.rs-0133 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0067
/* FP:errors.rs-0134 */ # [derive (Diagnostic)] # [diag (ast_passes_tilde_const_disallowed)] pub (crate) struct TildeConstDisallowed { # [primary_span] pub span : Span , # [subdiagnostic] pub reason : TildeConstReason , }
/* FP:errors.rs-0135 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_ENUM_0068
/* FP:errors.rs-0136 */ # [derive (Subdiagnostic , Copy , Clone)] pub (crate) enum TildeConstReason { # [note (ast_passes_closure)] Closure , # [note (ast_passes_function)] Function { # [primary_span] ident : Span , } , # [note (ast_passes_trait)] Trait { # [primary_span] span : Span , } , # [note (ast_passes_trait_impl)] TraitImpl { # [primary_span] span : Span , } , # [note (ast_passes_impl)] Impl { # [primary_span] span : Span , } , # [note (ast_passes_trait_assoc_ty)] TraitAssocTy { # [primary_span] span : Span , } , # [note (ast_passes_trait_impl_assoc_ty)] TraitImplAssocTy { # [primary_span] span : Span , } , # [note (ast_passes_inherent_assoc_ty)] InherentAssocTy { # [primary_span] span : Span , } , # [note (ast_passes_struct)] Struct { # [primary_span] span : Span , } , # [note (ast_passes_enum)] Enum { # [primary_span] span : Span , } , # [note (ast_passes_union)] Union { # [primary_span] span : Span , } , # [note (ast_passes_anon_const)] AnonConst { # [primary_span] span : Span , } , # [note (ast_passes_object)] TraitObject , # [note (ast_passes_item)] Item , }
/* FP:errors.rs-0137 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0069
/* FP:errors.rs-0138 */ # [derive (Diagnostic)] # [diag (ast_passes_const_and_coroutine)] pub (crate) struct ConstAndCoroutine { # [primary_span] pub spans : Vec < Span > , # [label (ast_passes_const)] pub const_span : Span , # [label (ast_passes_coroutine)] pub coroutine_span : Span , # [label] pub span : Span , pub coroutine_kind : & 'static str , }
/* FP:errors.rs-0139 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0070
/* FP:errors.rs-0140 */ # [derive (Diagnostic)] # [diag (ast_passes_const_and_c_variadic)] pub (crate) struct ConstAndCVariadic { # [primary_span] pub spans : Vec < Span > , # [label (ast_passes_const)] pub const_span : Span , # [label (ast_passes_variadic)] pub variadic_span : Span , }
/* FP:errors.rs-0141 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0071
/* FP:errors.rs-0142 */ # [derive (Diagnostic)] # [diag (ast_passes_coroutine_and_c_variadic)] pub (crate) struct CoroutineAndCVariadic { # [primary_span] pub spans : Vec < Span > , pub coroutine_kind : & 'static str , # [label (ast_passes_const)] pub coroutine_span : Span , # [label (ast_passes_variadic)] pub variadic_span : Span , }
/* FP:errors.rs-0143 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0072
/* FP:errors.rs-0144 */ # [derive (Diagnostic)] # [diag (ast_passes_pattern_in_foreign , code = E0130)] pub (crate) struct PatternInForeign { # [primary_span] # [label] pub span : Span , }
/* FP:errors.rs-0145 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0073
/* FP:errors.rs-0146 */ # [derive (Diagnostic)] # [diag (ast_passes_pattern_in_bodiless , code = E0642)] pub (crate) struct PatternInBodiless { # [primary_span] # [label] pub span : Span , }
/* FP:errors.rs-0147 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0074
/* FP:errors.rs-0148 */ # [derive (Diagnostic)] # [diag (ast_passes_equality_in_where)] # [note] pub (crate) struct EqualityInWhere { # [primary_span] # [label] pub span : Span , # [subdiagnostic] pub assoc : Option < AssociatedSuggestion > , # [subdiagnostic] pub assoc2 : Option < AssociatedSuggestion2 > , }
/* FP:errors.rs-0149 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0075
/* FP:errors.rs-0150 */ # [derive (Subdiagnostic)] # [suggestion (ast_passes_suggestion , code = "{param}: {path}" , style = "verbose" , applicability = "maybe-incorrect")] pub (crate) struct AssociatedSuggestion { # [primary_span] pub span : Span , pub ident : Ident , pub param : Ident , pub path : String , }
/* FP:errors.rs-0151 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0076
/* FP:errors.rs-0152 */ # [derive (Subdiagnostic)] # [multipart_suggestion (ast_passes_suggestion_path , applicability = "maybe-incorrect")] pub (crate) struct AssociatedSuggestion2 { # [suggestion_part (code = "{args}")] pub span : Span , pub args : String , # [suggestion_part (code = "")] pub predicate : Span , pub trait_segment : Ident , pub potential_assoc : Ident , }
/* FP:errors.rs-0153 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0077
/* FP:errors.rs-0154 */ # [derive (Diagnostic)] # [diag (ast_passes_feature_on_non_nightly , code = E0554)] pub (crate) struct FeatureOnNonNightly { # [primary_span] pub span : Span , pub channel : & 'static str , # [subdiagnostic] pub stable_features : Vec < StableFeature > , # [suggestion (code = "" , applicability = "machine-applicable")] pub sugg : Option < Span > , }
/* FP:errors.rs-0155 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0078
/* FP:errors.rs-0156 */ pub (crate) struct StableFeature { pub name : Symbol , pub since : Symbol , }
/* FP:errors.rs-0157 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_IMPL_0079
/* FP:errors.rs-0158 */ impl Subdiagnostic for StableFeature { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . arg ("name" , self . name) ; diag . arg ("since" , self . since) ; diag . help (fluent :: ast_passes_stable_since) ; } }
/* FP:errors.rs-0159 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0080
/* FP:errors.rs-0160 */ # [derive (Diagnostic)] # [diag (ast_passes_incompatible_features)] # [help] pub (crate) struct IncompatibleFeatures { # [primary_span] pub spans : Vec < Span > , pub f1 : Symbol , pub f2 : Symbol , }
/* FP:errors.rs-0161 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0081
/* FP:errors.rs-0162 */ # [derive (Diagnostic)] # [diag (ast_passes_negative_bound_not_supported)] pub (crate) struct NegativeBoundUnsupported { # [primary_span] pub span : Span , }
/* FP:errors.rs-0163 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0082
/* FP:errors.rs-0164 */ # [derive (Diagnostic)] # [diag (ast_passes_constraint_on_negative_bound)] pub (crate) struct ConstraintOnNegativeBound { # [primary_span] pub span : Span , }
/* FP:errors.rs-0165 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0083
/* FP:errors.rs-0166 */ # [derive (Diagnostic)] # [diag (ast_passes_negative_bound_with_parenthetical_notation)] pub (crate) struct NegativeBoundWithParentheticalNotation { # [primary_span] pub span : Span , }
/* FP:errors.rs-0167 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0084
/* FP:errors.rs-0168 */ # [derive (Diagnostic)] # [diag (ast_passes_match_arm_with_no_body)] pub (crate) struct MatchArmWithNoBody { # [primary_span] pub span : Span , # [suggestion (code = " => {{ todo!() }}" , applicability = "has-placeholders" , style = "verbose")] pub suggestion : Span , }
/* FP:errors.rs-0169 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0085
/* FP:errors.rs-0170 */ # [derive (Diagnostic)] # [diag (ast_passes_precise_capturing_not_allowed_here)] pub (crate) struct PreciseCapturingNotAllowedHere { # [primary_span] pub span : Span , pub loc : & 'static str , }
/* FP:errors.rs-0171 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0086
/* FP:errors.rs-0172 */ # [derive (Diagnostic)] # [diag (ast_passes_precise_capturing_duplicated)] pub (crate) struct DuplicatePreciseCapturing { # [primary_span] pub bound1 : Span , # [label] pub bound2 : Span , }
/* FP:errors.rs-0173 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0087
/* FP:errors.rs-0174 */ # [derive (Diagnostic)] # [diag (ast_passes_extern_without_abi)] # [help] pub (crate) struct MissingAbi { # [primary_span] # [suggestion (code = "extern \"<abi>\"" , applicability = "has-placeholders")] pub span : Span , }
/* FP:errors.rs-0175 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0088
/* FP:errors.rs-0176 */ # [derive (LintDiagnostic)] # [diag (ast_passes_extern_without_abi_sugg)] pub (crate) struct MissingAbiSugg { # [suggestion (code = "extern {default_abi}" , applicability = "machine-applicable")] pub span : Span , pub default_abi : ExternAbi , }
/* FP:errors.rs-0177 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0089
/* FP:errors.rs-0178 */ # [derive (Diagnostic)] # [diag (ast_passes_abi_custom_safe_foreign_function)] pub (crate) struct AbiCustomSafeForeignFunction { # [primary_span] pub span : Span , # [suggestion (ast_passes_suggestion , applicability = "maybe-incorrect" , code = "" , style = "verbose")] pub safe_span : Span , }
/* FP:errors.rs-0179 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0090
/* FP:errors.rs-0180 */ # [derive (Diagnostic)] # [diag (ast_passes_abi_custom_safe_function)] pub (crate) struct AbiCustomSafeFunction { # [primary_span] pub span : Span , pub abi : ExternAbi , # [suggestion (ast_passes_suggestion , applicability = "maybe-incorrect" , code = "unsafe " , style = "verbose")] pub unsafe_span : Span , }
/* FP:errors.rs-0181 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0091
/* FP:errors.rs-0182 */ # [derive (Diagnostic)] # [diag (ast_passes_abi_cannot_be_coroutine)] pub (crate) struct AbiCannotBeCoroutine { # [primary_span] pub span : Span , pub abi : ExternAbi , # [suggestion (ast_passes_suggestion , applicability = "maybe-incorrect" , code = "" , style = "verbose")] pub coroutine_kind_span : Span , pub coroutine_kind_str : & 'static str , }
/* FP:errors.rs-0183 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0092
/* FP:errors.rs-0184 */ # [derive (Diagnostic)] # [diag (ast_passes_abi_must_not_have_parameters_or_return_type)] # [note] pub (crate) struct AbiMustNotHaveParametersOrReturnType { # [primary_span] pub spans : Vec < Span > , pub abi : ExternAbi , # [suggestion (ast_passes_suggestion , applicability = "maybe-incorrect" , code = "{padding}fn {symbol}()" , style = "verbose")] pub suggestion_span : Span , pub symbol : Symbol , pub padding : & 'static str , }
/* FP:errors.rs-0185 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0093
/* FP:errors.rs-0186 */ # [derive (Diagnostic)] # [diag (ast_passes_abi_must_not_have_return_type)] # [note] pub (crate) struct AbiMustNotHaveReturnType { # [primary_span] # [help] pub span : Span , pub abi : ExternAbi , }
/* FP:errors.rs-0187 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_passes_src_errors_STRUCT_0094
/* FP:errors.rs-0188 */ # [derive (Diagnostic)] # [diag (ast_passes_abi_x86_interrupt)] # [note] pub (crate) struct AbiX86Interrupt { # [primary_span] pub spans : Vec < Span > , pub param_count : usize , }