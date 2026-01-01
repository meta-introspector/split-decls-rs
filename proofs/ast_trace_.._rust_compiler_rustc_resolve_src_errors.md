# AST Trace: ../rust/compiler/rustc_resolve/src/errors.rs

Generated 127 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
use rustc_errors::codes::*;
use rustc_errors::{
    Applicability, Diag, ElidedLifetimeInPathSubdiag, EmissionGuarantee, IntoDiagArg, MultiSpan,
    Subdiagnostic,
};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_macros::{Diagnostic, Subdiagnostic};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_span::{Ident, Span, Symbol};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::late::PatternSource;
use crate::{Res, fluent_generated as fluent};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=17

```rust
#[derive(Diagnostic)]
#[diag(resolve_generic_params_from_outer_item, code = E0401)]
pub(crate) struct GenericParamsFromOuterItem {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[subdiagnostic]
    pub(crate) label: Option<GenericParamsFromOuterItemLabel>,
    #[label(resolve_refer_to_type_directly)]
    pub(crate) refer_to_type_directly: Option<Span>,
    #[subdiagnostic]
    pub(crate) sugg: Option<GenericParamsFromOuterItemSugg>,
    #[subdiagnostic]
    pub(crate) static_or_const: Option<GenericParamsFromOuterItemStaticOrConst>,
    pub(crate) is_self: bool,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Subdiagnostic)]
pub(crate) enum GenericParamsFromOuterItemStaticOrConst {
    #[note(resolve_generic_params_from_outer_item_static)]
    Static,
    #[note(resolve_generic_params_from_outer_item_const)]
    Const,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
#[derive(Subdiagnostic)]
pub(crate) enum GenericParamsFromOuterItemLabel {
    #[label(resolve_generic_params_from_outer_item_self_ty_param)]
    SelfTyParam(#[primary_span] Span),
    #[label(resolve_generic_params_from_outer_item_self_ty_alias)]
    SelfTyAlias(#[primary_span] Span),
    #[label(resolve_generic_params_from_outer_item_ty_param)]
    TyParam(#[primary_span] Span),
    #[label(resolve_generic_params_from_outer_item_const_param)]
    ConstParam(#[primary_span] Span),
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=8

```rust
#[derive(Subdiagnostic)]
#[suggestion(resolve_suggestion, code = "{snippet}", applicability = "maybe-incorrect")]
pub(crate) struct GenericParamsFromOuterItemSugg {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) snippet: String,
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Diagnostic)]
#[diag(resolve_name_is_already_used_as_generic_parameter, code = E0403)]
pub(crate) struct NameAlreadyUsedInParameterList {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[label(resolve_first_use_of_name)]
    pub(crate) first_use_span: Span,
    pub(crate) name: Ident,
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
#[derive(Diagnostic)]
#[diag(resolve_method_not_member_of_trait, code = E0407)]
pub(crate) struct MethodNotMemberOfTrait {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) method: Ident,
    pub(crate) trait_: String,
    #[subdiagnostic]
    pub(crate) sub: Option<AssociatedFnWithSimilarNameExists>,
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=12

```rust
#[derive(Subdiagnostic)]
#[suggestion(
    resolve_associated_fn_with_similar_name_exists,
    code = "{candidate}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct AssociatedFnWithSimilarNameExists {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) candidate: Symbol,
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
#[derive(Diagnostic)]
#[diag(resolve_type_not_member_of_trait, code = E0437)]
pub(crate) struct TypeNotMemberOfTrait {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) type_: Ident,
    pub(crate) trait_: String,
    #[subdiagnostic]
    pub(crate) sub: Option<AssociatedTypeWithSimilarNameExists>,
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=12

```rust
#[derive(Subdiagnostic)]
#[suggestion(
    resolve_associated_type_with_similar_name_exists,
    code = "{candidate}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct AssociatedTypeWithSimilarNameExists {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) candidate: Symbol,
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
#[derive(Diagnostic)]
#[diag(resolve_const_not_member_of_trait, code = E0438)]
pub(crate) struct ConstNotMemberOfTrait {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) const_: Ident,
    pub(crate) trait_: String,
    #[subdiagnostic]
    pub(crate) sub: Option<AssociatedConstWithSimilarNameExists>,
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=12

```rust
#[derive(Subdiagnostic)]
#[suggestion(
    resolve_associated_const_with_similar_name_exists,
    code = "{candidate}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct AssociatedConstWithSimilarNameExists {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) candidate: Symbol,
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Diagnostic)]
#[diag(resolve_variable_bound_with_different_mode, code = E0409)]
pub(crate) struct VariableBoundWithDifferentMode {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[label(resolve_first_binding_span)]
    pub(crate) first_binding_span: Span,
    pub(crate) variable_name: Ident,
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_ident_bound_more_than_once_in_parameter_list, code = E0415)]
pub(crate) struct IdentifierBoundMoreThanOnceInParameterList {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) identifier: Ident,
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_ident_bound_more_than_once_in_same_pattern, code = E0416)]
pub(crate) struct IdentifierBoundMoreThanOnceInSamePattern {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) identifier: Ident,
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=15

```rust
#[derive(Diagnostic)]
#[diag(resolve_undeclared_label, code = E0426)]
pub(crate) struct UndeclaredLabel {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
    #[subdiagnostic]
    pub(crate) sub_reachable: Option<LabelWithSimilarNameReachable>,
    #[subdiagnostic]
    pub(crate) sub_reachable_suggestion: Option<TryUsingSimilarlyNamedLabel>,
    #[subdiagnostic]
    pub(crate) sub_unreachable: Option<UnreachableLabelWithSimilarNameExists>,
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=16

```rust
#[derive(Subdiagnostic)]
#[label(resolve_label_with_similar_name_reachable)]
pub(crate) struct LabelWithSimilarNameReachable(#[primary_span] pub(crate) Span);

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_try_using_similarly_named_label,
    code = "{ident_name}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct TryUsingSimilarlyNamedLabel {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident_name: Symbol,
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic)]
#[label(resolve_unreachable_label_with_similar_name_exists)]
pub(crate) struct UnreachableLabelWithSimilarNameExists {
    #[primary_span]
    pub(crate) ident_span: Span,
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(resolve_self_import_can_only_appear_once_in_the_list, code = E0430)]
pub(crate) struct SelfImportCanOnlyAppearOnceInTheList {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(resolve_self_import_only_in_import_list_with_non_empty_prefix, code = E0431)]
pub(crate) struct SelfImportOnlyInImportListWithNonEmptyPrefix {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(resolve_cannot_capture_dynamic_environment_in_fn_item, code = E0434)]
#[help]
pub(crate) struct CannotCaptureDynamicEnvironmentInFnItem {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=13

```rust
#[derive(Diagnostic)]
#[diag(resolve_attempt_to_use_non_constant_value_in_constant, code = E0435)]
pub(crate) struct AttemptToUseNonConstantValueInConstant<'a> {
    #[primary_span]
    pub(crate) span: Span,
    #[subdiagnostic]
    pub(crate) with: Option<AttemptToUseNonConstantValueInConstantWithSuggestion<'a>>,
    #[subdiagnostic]
    pub(crate) with_label: Option<AttemptToUseNonConstantValueInConstantLabelWithSuggestion>,
    #[subdiagnostic]
    pub(crate) without: Option<AttemptToUseNonConstantValueInConstantWithoutSuggestion<'a>>,
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=16

```rust
#[derive(Subdiagnostic)]
#[multipart_suggestion(
    resolve_attempt_to_use_non_constant_value_in_constant_with_suggestion,
    style = "verbose",
    applicability = "has-placeholders"
)]
pub(crate) struct AttemptToUseNonConstantValueInConstantWithSuggestion<'a> {
    // #[primary_span]
    #[suggestion_part(code = "{suggestion} ")]
    pub(crate) span: Span,
    pub(crate) suggestion: &'a str,
    #[suggestion_part(code = ": /* Type */")]
    pub(crate) type_span: Option<Span>,
    pub(crate) current: &'a str,
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic)]
#[label(resolve_attempt_to_use_non_constant_value_in_constant_label_with_suggestion)]
pub(crate) struct AttemptToUseNonConstantValueInConstantLabelWithSuggestion {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Subdiagnostic)]
#[label(resolve_attempt_to_use_non_constant_value_in_constant_without_suggestion)]
pub(crate) struct AttemptToUseNonConstantValueInConstantWithoutSuggestion<'a> {
    #[primary_span]
    pub(crate) ident_span: Span,
    pub(crate) suggestion: &'a str,
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Diagnostic)]
#[diag(resolve_self_imports_only_allowed_within, code = E0429)]
pub(crate) struct SelfImportsOnlyAllowedWithin {
    #[primary_span]
    pub(crate) span: Span,
    #[subdiagnostic]
    pub(crate) suggestion: Option<SelfImportsOnlyAllowedWithinSuggestion>,
    #[subdiagnostic]
    pub(crate) mpart_suggestion: Option<SelfImportsOnlyAllowedWithinMultipartSuggestion>,
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Subdiagnostic)]
#[suggestion(
    resolve_self_imports_only_allowed_within_suggestion,
    code = "",
    applicability = "machine-applicable"
)]
pub(crate) struct SelfImportsOnlyAllowedWithinSuggestion {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=12

```rust
#[derive(Subdiagnostic)]
#[multipart_suggestion(
    resolve_self_imports_only_allowed_within_multipart_suggestion,
    applicability = "machine-applicable"
)]
pub(crate) struct SelfImportsOnlyAllowedWithinMultipartSuggestion {
    #[suggestion_part(code = "{{")]
    pub(crate) multipart_start: Span,
    #[suggestion_part(code = "}}")]
    pub(crate) multipart_end: Span,
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=17

```rust
#[derive(Diagnostic)]
#[diag(resolve_binding_shadows_something_unacceptable, code = E0530)]
pub(crate) struct BindingShadowsSomethingUnacceptable<'a> {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) shadowing_binding: PatternSource,
    pub(crate) shadowed_binding: Res,
    pub(crate) article: &'a str,
    #[subdiagnostic]
    pub(crate) sub_suggestion: Option<BindingShadowsSomethingUnacceptableSuggestion>,
    #[label(resolve_label_shadowed_binding)]
    pub(crate) shadowed_binding_span: Span,
    pub(crate) participle: &'a str,
    pub(crate) name: Symbol,
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=12

```rust
#[derive(Subdiagnostic)]
#[suggestion(
    resolve_binding_shadows_something_unacceptable_suggestion,
    code = "{name}(..)",
    applicability = "unspecified"
)]
pub(crate) struct BindingShadowsSomethingUnacceptableSuggestion {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_forward_declared_generic_param, code = E0128)]
pub(crate) struct ForwardDeclaredGenericParam {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) param: Symbol,
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_forward_declared_generic_in_const_param_ty)]
pub(crate) struct ForwardDeclaredGenericInConstParamTy {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) param: Symbol,
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_param_in_ty_of_const_param, code = E0770)]
pub(crate) struct ParamInTyOfConstParam {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(resolve_self_in_generic_param_default, code = E0735)]
pub(crate) struct SelfInGenericParamDefault {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(resolve_self_in_const_generic_ty)]
pub(crate) struct SelfInConstGenericTy {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=13

```rust
#[derive(Diagnostic)]
#[diag(resolve_param_in_non_trivial_anon_const)]
pub(crate) struct ParamInNonTrivialAnonConst {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
    #[subdiagnostic]
    pub(crate) param_kind: ParamKindInNonTrivialAnonConst,
    #[subdiagnostic]
    pub(crate) help: Option<ParamInNonTrivialAnonConstHelp>,
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=15

```rust
#[derive(Subdiagnostic)]
#[help(resolve_param_in_non_trivial_anon_const_help)]
pub(crate) struct ParamInNonTrivialAnonConstHelp;

#[derive(Debug)]
#[derive(Subdiagnostic)]
pub(crate) enum ParamKindInNonTrivialAnonConst {
    #[note(resolve_type_param_in_non_trivial_anon_const)]
    Type,
    #[help(resolve_const_param_in_non_trivial_anon_const)]
    Const { name: Symbol },
    #[note(resolve_lifetime_param_in_non_trivial_anon_const)]
    Lifetime,
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=18

```rust
#[derive(Diagnostic)]
#[diag(resolve_unreachable_label, code = E0767)]
#[note]
pub(crate) struct UnreachableLabel {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
    #[label(resolve_label_definition_span)]
    pub(crate) definition_span: Span,
    #[subdiagnostic]
    pub(crate) sub_suggestion: Option<UnreachableLabelSubSuggestion>,
    #[subdiagnostic]
    pub(crate) sub_suggestion_label: Option<UnreachableLabelSubLabel>,
    #[subdiagnostic]
    pub(crate) sub_unreachable_label: Option<UnreachableLabelSubLabelUnreachable>,
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=12

```rust
#[derive(Subdiagnostic)]
#[suggestion(
    resolve_unreachable_label_suggestion_use_similarly_named,
    code = "{ident_name}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct UnreachableLabelSubSuggestion {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident_name: Symbol,
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic)]
#[label(resolve_unreachable_label_similar_name_reachable)]
pub(crate) struct UnreachableLabelSubLabel {
    #[primary_span]
    pub(crate) ident_span: Span,
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic)]
#[label(resolve_unreachable_label_similar_name_unreachable)]
pub(crate) struct UnreachableLabelSubLabelUnreachable {
    #[primary_span]
    pub(crate) ident_span: Span,
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_invalid_asm_sym)]
#[help]
pub(crate) struct InvalidAsmSym {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(resolve_lowercase_self)]
pub(crate) struct LowercaseSelf {
    #[primary_span]
    #[suggestion(code = "Self", applicability = "maybe-incorrect", style = "short")]
    pub(crate) span: Span,
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Debug)]
#[derive(Diagnostic)]
#[diag(resolve_binding_in_never_pattern)]
pub(crate) struct BindingInNeverPattern {
    #[primary_span]
    #[suggestion(code = "_", applicability = "machine-applicable", style = "short")]
    pub(crate) span: Span,
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=13

```rust
#[derive(Diagnostic)]
#[diag(resolve_trait_impl_duplicate, code = E0201)]
pub(crate) struct TraitImplDuplicate {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[label(resolve_old_span_label)]
    pub(crate) old_span: Span,
    #[label(resolve_trait_item_span)]
    pub(crate) trait_item_span: Span,
    pub(crate) name: Ident,
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(resolve_relative_2018)]
pub(crate) struct Relative2018 {
    #[primary_span]
    pub(crate) span: Span,
    #[suggestion(code = "crate::{path_str}", applicability = "maybe-incorrect")]
    pub(crate) path_span: Span,
    pub(crate) path_str: String,
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=14

```rust
#[derive(Diagnostic)]
#[diag(resolve_ancestor_only, code = E0742)]
pub(crate) struct AncestorOnly(#[primary_span] pub(crate) Span);

#[derive(Diagnostic)]
#[diag(resolve_expected_module_found, code = E0577)]
pub(crate) struct ExpectedModuleFound {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) res: Res,
    pub(crate) path_str: String,
}
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=13

```rust
#[derive(Diagnostic)]
#[diag(resolve_indeterminate, code = E0578)]
pub(crate) struct Indeterminate(#[primary_span] pub(crate) Span);

#[derive(Diagnostic)]
#[diag(resolve_tool_module_imported)]
pub(crate) struct ToolModuleImported {
    #[primary_span]
    pub(crate) span: Span,
    #[note]
    pub(crate) import: Span,
}
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=20

```rust
#[derive(Diagnostic)]
#[diag(resolve_module_only)]
pub(crate) struct ModuleOnly(#[primary_span] pub(crate) Span);

#[derive(Diagnostic)]
#[diag(resolve_macro_expected_found)]
pub(crate) struct MacroExpectedFound<'a> {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) found: &'a str,
    pub(crate) article: &'static str,
    pub(crate) expected: &'a str,
    pub(crate) macro_path: &'a str,
    #[subdiagnostic]
    pub(crate) remove_surrounding_derive: Option<RemoveSurroundingDerive>,
    #[subdiagnostic]
    pub(crate) add_as_non_derive: Option<AddAsNonDerive<'a>>,
}
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic)]
#[help(resolve_remove_surrounding_derive)]
pub(crate) struct RemoveSurroundingDerive {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 54
**Metadata**: AST_ID=54 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
#[derive(Subdiagnostic)]
#[help(resolve_add_as_non_derive)]
pub(crate) struct AddAsNonDerive<'a> {
    pub(crate) macro_path: &'a str,
}
```

## Block 55
**Metadata**: AST_ID=55 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_proc_macro_same_crate)]
pub(crate) struct ProcMacroSameCrate {
    #[primary_span]
    pub(crate) span: Span,
    #[help]
    pub(crate) is_test: bool,
}
```

## Block 56
**Metadata**: AST_ID=56 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(resolve_imported_crate)]
pub(crate) struct CrateImported {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 57
**Metadata**: AST_ID=57 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(resolve_macro_use_extern_crate_self)]
pub(crate) struct MacroUseExternCrateSelf {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 58
**Metadata**: AST_ID=58 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(resolve_accessible_unsure)]
#[note]
pub(crate) struct CfgAccessibleUnsure {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 59
**Metadata**: AST_ID=59 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
#[derive(Debug)]
#[derive(Diagnostic)]
#[diag(resolve_param_in_enum_discriminant)]
pub(crate) struct ParamInEnumDiscriminant {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
    #[subdiagnostic]
    pub(crate) param_kind: ParamKindInEnumDiscriminant,
}
```

## Block 60
**Metadata**: AST_ID=60 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Debug)]
#[derive(Subdiagnostic)]
pub(crate) enum ParamKindInEnumDiscriminant {
    #[note(resolve_type_param_in_enum_discriminant)]
    Type,
    #[note(resolve_const_param_in_enum_discriminant)]
    Const,
    #[note(resolve_lifetime_param_in_enum_discriminant)]
    Lifetime,
}
```

## Block 61
**Metadata**: AST_ID=61 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic)]
#[label(resolve_change_import_binding)]
pub(crate) struct ChangeImportBinding {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 62
**Metadata**: AST_ID=62 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=12

```rust
#[derive(Subdiagnostic)]
#[suggestion(
    resolve_change_import_binding,
    code = "{suggestion}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct ChangeImportBindingSuggestion {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) suggestion: String,
}
```

## Block 63
**Metadata**: AST_ID=63 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(resolve_imports_cannot_refer_to)]
pub(crate) struct ImportsCannotReferTo<'a> {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) what: &'a str,
}
```

## Block 64
**Metadata**: AST_ID=64 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_cannot_find_ident_in_this_scope)]
pub(crate) struct CannotFindIdentInThisScope<'a> {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) expected: &'a str,
    pub(crate) ident: Ident,
}
```

## Block 65
**Metadata**: AST_ID=65 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Subdiagnostic)]
#[note(resolve_explicit_unsafe_traits)]
pub(crate) struct ExplicitUnsafeTraits {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}
```

## Block 66
**Metadata**: AST_ID=66 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic)]
#[note(resolve_macro_defined_later)]
pub(crate) struct MacroDefinedLater {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 67
**Metadata**: AST_ID=67 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Subdiagnostic)]
#[label(resolve_consider_move_macro_position)]
pub(crate) struct MacroSuggMovePosition {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}
```

## Block 68
**Metadata**: AST_ID=68 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=22

```rust
#[derive(Subdiagnostic)]
pub(crate) enum MacroRulesNot {
    #[label(resolve_macro_cannot_use_as_fn_like)]
    Func {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[label(resolve_macro_cannot_use_as_attr)]
    Attr {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[label(resolve_macro_cannot_use_as_derive)]
    Derive {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
}
```

## Block 69
**Metadata**: AST_ID=69 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic)]
#[note(resolve_missing_macro_rules_name)]
pub(crate) struct MaybeMissingMacroRulesName {
    #[primary_span]
    pub(crate) spans: MultiSpan,
}
```

## Block 70
**Metadata**: AST_ID=70 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=16

```rust
#[derive(Subdiagnostic)]
#[help(resolve_added_macro_use)]
pub(crate) struct AddedMacroUse;

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_consider_adding_a_derive,
    code = "{suggestion}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct ConsiderAddingADerive {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) suggestion: String,
}
```

## Block 71
**Metadata**: AST_ID=71 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(resolve_cannot_determine_import_resolution)]
pub(crate) struct CannotDetermineImportResolution {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 72
**Metadata**: AST_ID=72 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(resolve_cannot_determine_macro_resolution)]
#[note]
pub(crate) struct CannotDetermineMacroResolution {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) kind: &'static str,
    pub(crate) path: String,
}
```

## Block 73
**Metadata**: AST_ID=73 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(resolve_cannot_be_reexported_private, code = E0364)]
pub(crate) struct CannotBeReexportedPrivate {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}
```

## Block 74
**Metadata**: AST_ID=74 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(resolve_cannot_be_reexported_crate_public, code = E0364)]
pub(crate) struct CannotBeReexportedCratePublic {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}
```

## Block 75
**Metadata**: AST_ID=75 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(resolve_cannot_be_reexported_private, code = E0365)]
#[note(resolve_consider_declaring_with_pub)]
pub(crate) struct CannotBeReexportedPrivateNS {
    #[primary_span]
    #[label(resolve_reexport_of_private)]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}
```

## Block 76
**Metadata**: AST_ID=76 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(resolve_cannot_be_reexported_crate_public, code = E0365)]
#[note(resolve_consider_declaring_with_pub)]
pub(crate) struct CannotBeReexportedCratePublicNS {
    #[primary_span]
    #[label(resolve_reexport_of_crate_public)]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}
```

## Block 77
**Metadata**: AST_ID=77 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic)]
#[help(resolve_consider_adding_macro_export)]
pub(crate) struct ConsiderAddingMacroExport {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 78
**Metadata**: AST_ID=78 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Subdiagnostic)]
#[suggestion(
    resolve_consider_marking_as_pub_crate,
    code = "pub(crate)",
    applicability = "maybe-incorrect"
)]
pub(crate) struct ConsiderMarkingAsPubCrate {
    #[primary_span]
    pub(crate) vis_span: Span,
}
```

## Block 79
**Metadata**: AST_ID=79 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Subdiagnostic)]
#[note(resolve_consider_marking_as_pub)]
pub(crate) struct ConsiderMarkingAsPub {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}
```

## Block 80
**Metadata**: AST_ID=80 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(resolve_cannot_glob_import_possible_crates)]
pub(crate) struct CannotGlobImportAllCrates {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 81
**Metadata**: AST_ID=81 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=13

```rust
#[derive(Subdiagnostic)]
#[suggestion(
    resolve_unexpected_res_change_ty_to_const_param_sugg,
    code = "const ",
    style = "verbose"
)]
pub(crate) struct UnexpectedResChangeTyToConstParamSugg {
    #[primary_span]
    pub span: Span,
    #[applicability]
    pub applicability: Applicability,
}
```

## Block 82
**Metadata**: AST_ID=82 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=14

```rust
#[derive(Subdiagnostic)]
#[suggestion(
    resolve_unexpected_res_use_at_op_in_slice_pat_with_range_sugg,
    code = "{snippet}",
    applicability = "maybe-incorrect",
    style = "verbose"
)]
pub(crate) struct UnexpectedResUseAtOpInSlicePatWithRangeSugg {
    #[primary_span]
    pub span: Span,
    pub ident: Ident,
    pub snippet: String,
}
```

## Block 83
**Metadata**: AST_ID=83 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(resolve_extern_crate_loading_macro_not_at_crate_root, code = E0468)]
pub(crate) struct ExternCrateLoadingMacroNotAtCrateRoot {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 84
**Metadata**: AST_ID=84 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(resolve_extern_crate_self_requires_renaming)]
pub(crate) struct ExternCrateSelfRequiresRenaming {
    #[primary_span]
    #[suggestion(code = "extern crate self as name;", applicability = "has-placeholders")]
    pub(crate) span: Span,
}
```

## Block 85
**Metadata**: AST_ID=85 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_macro_use_name_already_in_use)]
#[note]
pub(crate) struct MacroUseNameAlreadyInUse {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
}
```

## Block 86
**Metadata**: AST_ID=86 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(resolve_imported_macro_not_found, code = E0469)]
pub(crate) struct ImportedMacroNotFound {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 87
**Metadata**: AST_ID=87 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_macro_extern_deprecated)]
pub(crate) struct MacroExternDeprecated {
    #[primary_span]
    pub(crate) span: Span,
    #[help]
    pub inner_attribute: bool,
}
```

## Block 88
**Metadata**: AST_ID=88 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(resolve_arguments_macro_use_not_allowed)]
pub(crate) struct ArgumentsMacroUseNotAllowed {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 89
**Metadata**: AST_ID=89 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(resolve_unnamed_crate_root_import)]
pub(crate) struct UnnamedCrateRootImport {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 90
**Metadata**: AST_ID=90 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(resolve_macro_expanded_extern_crate_cannot_shadow_extern_arguments)]
pub(crate) struct MacroExpandedExternCrateCannotShadowExternArguments {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 91
**Metadata**: AST_ID=91 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(resolve_elided_anonymous_lifetime_report_error, code = E0637)]
pub(crate) struct ElidedAnonymousLifetimeReportError {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[subdiagnostic]
    pub(crate) suggestion: Option<ElidedAnonymousLifetimeReportErrorSuggestion>,
}
```

## Block 92
**Metadata**: AST_ID=92 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_lending_iterator_report_error)]
pub(crate) struct LendingIteratorReportError {
    #[primary_span]
    pub(crate) lifetime: Span,
    #[note]
    pub(crate) ty: Span,
}
```

## Block 93
**Metadata**: AST_ID=93 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(resolve_anonymous_lifetime_non_gat_report_error)]
pub(crate) struct AnonymousLifetimeNonGatReportError {
    #[primary_span]
    #[label]
    pub(crate) lifetime: Span,
}
```

## Block 94
**Metadata**: AST_ID=94 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
#[derive(Subdiagnostic)]
#[multipart_suggestion(
    resolve_elided_anonymous_lifetime_report_error_suggestion,
    applicability = "machine-applicable"
)]
pub(crate) struct ElidedAnonymousLifetimeReportErrorSuggestion {
    #[suggestion_part(code = "for<'a> ")]
    pub(crate) lo: Span,
    #[suggestion_part(code = "'a ")]
    pub(crate) hi: Span,
}
```

## Block 95
**Metadata**: AST_ID=95 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(resolve_explicit_anonymous_lifetime_report_error, code = E0637)]
pub(crate) struct ExplicitAnonymousLifetimeReportError {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}
```

## Block 96
**Metadata**: AST_ID=96 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_implicit_elided_lifetimes_not_allowed_here, code = E0726)]
pub(crate) struct ImplicitElidedLifetimeNotAllowedHere {
    #[primary_span]
    pub(crate) span: Span,
    #[subdiagnostic]
    pub(crate) subdiag: ElidedLifetimeInPathSubdiag,
}
```

## Block 97
**Metadata**: AST_ID=97 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_underscore_lifetime_is_reserved, code = E0637)]
#[help]
pub(crate) struct UnderscoreLifetimeIsReserved {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}
```

## Block 98
**Metadata**: AST_ID=98 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_static_lifetime_is_reserved, code = E0262)]
pub(crate) struct StaticLifetimeIsReserved {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) lifetime: Ident,
}
```

## Block 99
**Metadata**: AST_ID=99 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(resolve_variable_is_not_bound_in_all_patterns, code = E0408)]
pub(crate) struct VariableIsNotBoundInAllPatterns {
    #[primary_span]
    pub(crate) multispan: MultiSpan,
    pub(crate) name: Ident,
}
```

## Block 100
**Metadata**: AST_ID=100 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Subdiagnostic, Debug, Clone)]
#[label(resolve_pattern_doesnt_bind_name)]
pub(crate) struct PatternDoesntBindName {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) name: Ident,
}
```

## Block 101
**Metadata**: AST_ID=101 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic, Debug, Clone)]
#[label(resolve_variable_not_in_all_patterns)]
pub(crate) struct VariableNotInAllPatterns {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 102
**Metadata**: AST_ID=102 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=12

```rust
#[derive(Subdiagnostic)]
#[multipart_suggestion(
    resolve_variable_is_a_typo,
    applicability = "maybe-incorrect",
    style = "verbose"
)]
pub(crate) struct PatternBindingTypo {
    #[suggestion_part(code = "{typo}")]
    pub(crate) spans: Vec<Span>,
    pub(crate) typo: Symbol,
}
```

## Block 103
**Metadata**: AST_ID=103 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=15

```rust
#[derive(Diagnostic)]
#[diag(resolve_name_defined_multiple_time)]
#[note]
pub(crate) struct NameDefinedMultipleTime {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
    pub(crate) descr: &'static str,
    pub(crate) container: &'static str,
    #[subdiagnostic]
    pub(crate) label: NameDefinedMultipleTimeLabel,
    #[subdiagnostic]
    pub(crate) old_binding_label: Option<NameDefinedMultipleTimeOldBindingLabel>,
}
```

## Block 104
**Metadata**: AST_ID=104 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=14

```rust
#[derive(Subdiagnostic)]
pub(crate) enum NameDefinedMultipleTimeLabel {
    #[label(resolve_name_defined_multiple_time_reimported)]
    Reimported {
        #[primary_span]
        span: Span,
    },
    #[label(resolve_name_defined_multiple_time_redefined)]
    Redefined {
        #[primary_span]
        span: Span,
    },
}
```

## Block 105
**Metadata**: AST_ID=105 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=16

```rust
#[derive(Subdiagnostic)]
pub(crate) enum NameDefinedMultipleTimeOldBindingLabel {
    #[label(resolve_name_defined_multiple_time_old_binding_import)]
    Import {
        #[primary_span]
        span: Span,
        old_kind: &'static str,
    },
    #[label(resolve_name_defined_multiple_time_old_binding_definition)]
    Definition {
        #[primary_span]
        span: Span,
        old_kind: &'static str,
    },
}
```

## Block 106
**Metadata**: AST_ID=106 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(resolve_is_private, code = E0603)]
pub(crate) struct IsPrivate<'a> {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) ident_descr: &'a str,
    pub(crate) ident: Ident,
}
```

## Block 107
**Metadata**: AST_ID=107 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(resolve_generic_arguments_in_macro_path)]
pub(crate) struct GenericArgumentsInMacroPath {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 108
**Metadata**: AST_ID=108 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(resolve_attributes_starting_with_rustc_are_reserved)]
pub(crate) struct AttributesStartingWithRustcAreReserved {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 109
**Metadata**: AST_ID=109 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Diagnostic)]
#[diag(resolve_cannot_use_through_an_import)]
pub(crate) struct CannotUseThroughAnImport {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) article: &'static str,
    pub(crate) descr: &'static str,
    #[note]
    pub(crate) binding_span: Option<Span>,
}
```

## Block 110
**Metadata**: AST_ID=110 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(resolve_name_reserved_in_attribute_namespace)]
pub(crate) struct NameReservedInAttributeNamespace {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}
```

## Block 111
**Metadata**: AST_ID=111 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(resolve_cannot_find_builtin_macro_with_name)]
pub(crate) struct CannotFindBuiltinMacroWithName {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}
```

## Block 112
**Metadata**: AST_ID=112 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(resolve_tool_was_already_registered)]
pub(crate) struct ToolWasAlreadyRegistered {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) tool: Ident,
    #[label]
    pub(crate) old_ident_span: Span,
}
```

## Block 113
**Metadata**: AST_ID=113 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(resolve_tool_only_accepts_identifiers)]
pub(crate) struct ToolOnlyAcceptsIdentifiers {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) tool: Symbol,
}
```

## Block 114
**Metadata**: AST_ID=114 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=18

```rust
#[derive(Subdiagnostic)]
pub(crate) enum DefinedHere {
    #[label(resolve_similarly_named_defined_here)]
    SimilarlyNamed {
        #[primary_span]
        span: Span,
        candidate_descr: &'static str,
        candidate: Symbol,
    },
    #[label(resolve_single_item_defined_here)]
    SingleItem {
        #[primary_span]
        span: Span,
        candidate_descr: &'static str,
        candidate: Symbol,
    },
}
```

## Block 115
**Metadata**: AST_ID=115 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Subdiagnostic)]
#[label(resolve_outer_ident_is_not_publicly_reexported)]
pub(crate) struct OuterIdentIsNotPubliclyReexported {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) outer_ident_descr: &'static str,
    pub(crate) outer_ident: Ident,
}
```

## Block 116
**Metadata**: AST_ID=116 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic)]
#[label(resolve_constructor_private_if_any_field_private)]
pub(crate) struct ConstructorPrivateIfAnyFieldPrivate {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 117
**Metadata**: AST_ID=117 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
#[derive(Subdiagnostic)]
#[multipart_suggestion(
    resolve_consider_making_the_field_public,
    applicability = "maybe-incorrect",
    style = "verbose"
)]
pub(crate) struct ConsiderMakingTheFieldPublic {
    #[suggestion_part(code = "pub ")]
    pub(crate) spans: Vec<Span>,
    pub(crate) number_of_fields: usize,
}
```

## Block 118
**Metadata**: AST_ID=118 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=7 | LINES=28

```rust
#[derive(Subdiagnostic)]
pub(crate) enum ImportIdent {
    #[suggestion(
        resolve_suggestion_import_ident_through_reexport,
        code = "{path}",
        applicability = "machine-applicable",
        style = "verbose"
    )]
    ThroughReExport {
        #[primary_span]
        span: Span,
        ident: Ident,
        path: String,
    },
    #[suggestion(
        resolve_suggestion_import_ident_directly,
        code = "{path}",
        applicability = "machine-applicable",
        style = "verbose"
    )]
    Directly {
        #[primary_span]
        span: Span,
        ident: Ident,
        path: String,
    },
}
```

## Block 119
**Metadata**: AST_ID=119 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Subdiagnostic)]
#[note(resolve_note_and_refers_to_the_item_defined_here)]
pub(crate) struct NoteAndRefersToTheItemDefinedHere<'a> {
    #[primary_span]
    pub(crate) span: MultiSpan,
    pub(crate) binding_descr: &'a str,
    pub(crate) binding_name: Ident,
    pub(crate) first: bool,
    pub(crate) dots: bool,
}
```

## Block 120
**Metadata**: AST_ID=120 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic)]
#[suggestion(resolve_remove_unnecessary_import, code = "", applicability = "maybe-incorrect")]
pub(crate) struct RemoveUnnecessaryImport {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 121
**Metadata**: AST_ID=121 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
#[derive(Subdiagnostic)]
#[suggestion(
    resolve_remove_unnecessary_import,
    code = "",
    applicability = "maybe-incorrect",
    style = "tool-only"
)]
pub(crate) struct ToolOnlyRemoveUnnecessaryImport {
    #[primary_span]
    pub(crate) span: Span,
}
```

## Block 122
**Metadata**: AST_ID=122 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Subdiagnostic)]
#[note(resolve_ident_imported_here_but_it_is_desc)]
pub(crate) struct IdentImporterHereButItIsDesc<'a> {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) imported_ident: Ident,
    pub(crate) imported_ident_desc: &'a str,
}
```

## Block 123
**Metadata**: AST_ID=123 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic)]
#[note(resolve_ident_in_scope_but_it_is_desc)]
pub(crate) struct IdentInScopeButItIsDesc<'a> {
    pub(crate) imported_ident: Ident,
    pub(crate) imported_ident_desc: &'a str,
}
```

## Block 124
**Metadata**: AST_ID=124 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
pub(crate) struct FoundItemConfigureOut {
    pub(crate) span: Span,
    pub(crate) item_was: ItemWas,
}
```

## Block 125
**Metadata**: AST_ID=125 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=5

```rust
pub(crate) enum ItemWas {
    BehindFeature { feature: Symbol, span: Span },
    CfgOut { span: Span },
}
```

## Block 126
**Metadata**: AST_ID=126 | TYPE=FUNCTION | NAME=add_to_diag | COMPLEXITY=14 | LINES=21

```rust
impl Subdiagnostic for FoundItemConfigureOut {
    fn add_to_diag<G: EmissionGuarantee>(self, diag: &mut Diag<'_, G>) {
        let mut multispan: MultiSpan = self.span.into();
        match self.item_was {
            ItemWas::BehindFeature { feature, span } => {
                let key = "feature".into();
                let value = feature.into_diag_arg(&mut None);
                let msg = diag.dcx.eagerly_translate_to_string(
                    fluent::resolve_item_was_behind_feature,
                    [(&key, &value)].into_iter(),
                );
                multispan.push_span_label(span, msg);
            }
            ItemWas::CfgOut { span } => {
                multispan.push_span_label(span, fluent::resolve_item_was_cfg_out);
            }
        }
        diag.span_note(multispan, fluent::resolve_found_an_item_configured_out);
    }
}
```

## Block 127
**Metadata**: AST_ID=127 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=5 | LINES=13

```rust
#[derive(Diagnostic)]
#[diag(resolve_trait_impl_mismatch)]
pub(crate) struct TraitImplMismatch {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) name: Ident,
    pub(crate) kind: &'static str,
    pub(crate) trait_path: String,
    #[label(resolve_trait_impl_mismatch_label_item)]
    pub(crate) trait_item_span: Span,
}
```

---
*Generated by AST tracing system*
