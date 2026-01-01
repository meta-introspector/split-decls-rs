# AST Trace: ../rust/compiler/rustc_passes/src/errors.rs

Generated 160 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use std::io::Error;
use std::path::{Path, PathBuf};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
use crate::rustc_complete::codes::*;
use crate::rustc_complete::{
    Applicability, Diag, DiagCtxtHandle, DiagSymbolList, Diagnostic, EmissionGuarantee, Level,
    MultiSpan, Subdiagnostic,
};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::rustc_complete::Target;
use crate::rustc_complete::attrs::{MirDialect, MirPhase};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_macros::{Diagnostic, LintDiagnostic, Subdiagnostic};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::rustc_complete::ty::{MainDefinition, Ty};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::rustc_complete::{DUMMY_SP, Span, Symbol};
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=20

```rust
use crate::check_attr::ProcMacroKind;
use crate::fluent_generated as fluent;
use crate::lang_items::Duplicate;

#[derive(LintDiagnostic)]
#[diag(passes_incorrect_do_not_recommend_location)]
pub(crate) struct IncorrectDoNotRecommendLocation;

#[derive(LintDiagnostic)]
#[diag(passes_incorrect_do_not_recommend_args)]
pub(crate) struct DoNotRecommendDoesNotExpectArgs;

#[derive(Diagnostic)]
#[diag(passes_autodiff_attr)]
pub(crate) struct AutoDiffAttr {
    #[primary_span]
    #[label]
    pub attr_span: Span,
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_loop_match_attr)]
pub(crate) struct LoopMatchAttr {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub node_span: Span,
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_const_continue_attr)]
pub(crate) struct ConstContinueAttr {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub node_span: Span,
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
#[derive(LintDiagnostic)]
#[diag(passes_mixed_export_name_and_no_mangle)]
pub(crate) struct MixedExportNameAndNoMangle {
    #[label]
    #[suggestion(style = "verbose", code = "", applicability = "machine-applicable")]
    pub no_mangle_span: Span,
    #[note]
    pub export_name_span: Span,
    pub no_mangle_attr: &'static str,
    pub export_name_attr: &'static str,
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(LintDiagnostic)]
#[diag(passes_outer_crate_level_attr)]
pub(crate) struct OuterCrateLevelAttr {
    #[subdiagnostic]
    pub suggestion: OuterCrateLevelAttrSuggestion,
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic)]
#[multipart_suggestion(passes_outer_crate_level_attr_suggestion, style = "verbose")]
pub(crate) struct OuterCrateLevelAttrSuggestion {
    #[suggestion_part(code = "!")]
    pub bang_position: Span,
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(LintDiagnostic)]
#[diag(passes_inner_crate_level_attr)]
pub(crate) struct InnerCrateLevelAttr;

#[derive(LintDiagnostic)]
#[diag(passes_ignored_attr_with_macro)]
pub(crate) struct IgnoredAttrWithMacro<'a> {
    pub sym: &'a str,
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(passes_should_be_applied_to_fn)]
pub(crate) struct AttrShouldBeAppliedToFn {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub defn_span: Span,
    pub on_crate: bool,
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_non_exhaustive_with_default_field_values)]
pub(crate) struct NonExhaustiveWithDefaultFieldValues {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub defn_span: Span,
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_should_be_applied_to_trait)]
pub(crate) struct AttrShouldBeAppliedToTrait {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub defn_span: Span,
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_should_be_applied_to_static)]
pub(crate) struct AttrShouldBeAppliedToStatic {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub defn_span: Span,
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_expect_str)]
pub(crate) struct DocExpectStr<'a> {
    #[primary_span]
    pub attr_span: Span,
    pub attr_name: &'a str,
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_alias_empty)]
pub(crate) struct DocAliasEmpty<'a> {
    #[primary_span]
    pub span: Span,
    pub attr_str: &'a str,
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_alias_bad_char)]
pub(crate) struct DocAliasBadChar<'a> {
    #[primary_span]
    pub span: Span,
    pub attr_str: &'a str,
    pub char_: char,
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_alias_start_end)]
pub(crate) struct DocAliasStartEnd<'a> {
    #[primary_span]
    pub span: Span,
    pub attr_str: &'a str,
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_alias_bad_location)]
pub(crate) struct DocAliasBadLocation<'a> {
    #[primary_span]
    pub span: Span,
    pub attr_str: &'a str,
    pub location: &'a str,
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_alias_not_an_alias)]
pub(crate) struct DocAliasNotAnAlias<'a> {
    #[primary_span]
    pub span: Span,
    pub attr_str: &'a str,
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(LintDiagnostic)]
#[diag(passes_doc_alias_duplicated)]
pub(crate) struct DocAliasDuplicated {
    #[label]
    pub first_defn: Span,
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_alias_not_string_literal)]
pub(crate) struct DocAliasNotStringLiteral {
    #[primary_span]
    pub span: Span,
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_alias_malformed)]
pub(crate) struct DocAliasMalformed {
    #[primary_span]
    pub span: Span,
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_keyword_attribute_empty_mod)]
pub(crate) struct DocKeywordAttributeEmptyMod {
    #[primary_span]
    pub span: Span,
    pub attr_name: &'static str,
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_keyword_not_keyword)]
#[help]
pub(crate) struct DocKeywordNotKeyword {
    #[primary_span]
    pub span: Span,
    pub keyword: Symbol,
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_attribute_not_attribute)]
#[help]
pub(crate) struct DocAttributeNotAttribute {
    #[primary_span]
    pub span: Span,
    pub attribute: Symbol,
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_keyword_attribute_not_mod)]
pub(crate) struct DocKeywordAttributeNotMod {
    #[primary_span]
    pub span: Span,
    pub attr_name: &'static str,
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_fake_variadic_not_valid)]
pub(crate) struct DocFakeVariadicNotValid {
    #[primary_span]
    pub span: Span,
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_keyword_only_impl)]
pub(crate) struct DocKeywordOnlyImpl {
    #[primary_span]
    pub span: Span,
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_search_unbox_invalid)]
pub(crate) struct DocSearchUnboxInvalid {
    #[primary_span]
    pub span: Span,
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_inline_conflict)]
#[help]
pub(crate) struct DocKeywordConflict {
    #[primary_span]
    pub spans: MultiSpan,
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(LintDiagnostic)]
#[diag(passes_doc_inline_only_use)]
#[note]
pub(crate) struct DocInlineOnlyUse {
    #[label]
    pub attr_span: Span,
    #[label(passes_not_a_use_item_label)]
    pub item_span: Option<Span>,
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(LintDiagnostic)]
#[diag(passes_doc_masked_only_extern_crate)]
#[note]
pub(crate) struct DocMaskedOnlyExternCrate {
    #[label]
    pub attr_span: Span,
    #[label(passes_not_an_extern_crate_label)]
    pub item_span: Option<Span>,
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(LintDiagnostic)]
#[diag(passes_doc_masked_not_extern_crate_self)]
pub(crate) struct DocMaskedNotExternCrateSelf {
    #[label]
    pub attr_span: Span,
    #[label(passes_extern_crate_self_label)]
    pub item_span: Option<Span>,
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_doc_attr_not_crate_level)]
pub(crate) struct DocAttrNotCrateLevel<'a> {
    #[primary_span]
    pub span: Span,
    pub attr_name: &'a str,
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
#[derive(LintDiagnostic)]
#[diag(passes_doc_test_unknown)]
pub(crate) struct DocTestUnknown {
    pub path: String,
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=18

```rust
#[derive(LintDiagnostic)]
#[diag(passes_doc_test_literal)]
pub(crate) struct DocTestLiteral;

#[derive(LintDiagnostic)]
#[diag(passes_doc_test_takes_list)]
pub(crate) struct DocTestTakesList;

#[derive(LintDiagnostic)]
#[diag(passes_doc_cfg_hide_takes_list)]
pub(crate) struct DocCfgHideTakesList;

#[derive(LintDiagnostic)]
#[diag(passes_doc_test_unknown_any)]
pub(crate) struct DocTestUnknownAny {
    pub path: String,
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(LintDiagnostic)]
#[diag(passes_doc_test_unknown_spotlight)]
#[note]
#[note(passes_no_op_note)]
pub(crate) struct DocTestUnknownSpotlight {
    pub path: String,
    #[suggestion(style = "short", applicability = "machine-applicable", code = "notable_trait")]
    pub span: Span,
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(LintDiagnostic)]
#[diag(passes_doc_test_unknown_passes)]
#[note]
#[help]
#[note(passes_no_op_note)]
pub(crate) struct DocTestUnknownPasses {
    pub path: String,
    #[label]
    pub span: Span,
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(LintDiagnostic)]
#[diag(passes_doc_test_unknown_plugins)]
#[note]
#[note(passes_no_op_note)]
pub(crate) struct DocTestUnknownPlugins {
    pub path: String,
    #[label]
    pub span: Span,
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=10

```rust
#[derive(LintDiagnostic)]
#[diag(passes_doc_test_unknown_include)]
pub(crate) struct DocTestUnknownInclude {
    pub path: String,
    pub value: String,
    pub inner: &'static str,
    #[suggestion(code = "#{inner}[doc = include_str!(\"{value}\")]")]
    pub sugg: (Span, Applicability),
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=13

```rust
#[derive(LintDiagnostic)]
#[diag(passes_doc_invalid)]
pub(crate) struct DocInvalid;

#[derive(Diagnostic)]
#[diag(passes_has_incoherent_inherent_impl)]
pub(crate) struct HasIncoherentInherentImpl {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub span: Span,
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_both_ffi_const_and_pure, code = E0757)]
pub(crate) struct BothFfiConstAndPure {
    #[primary_span]
    pub attr_span: Span,
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_must_not_suspend)]
pub(crate) struct MustNotSuspend {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub span: Span,
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(LintDiagnostic)]
#[diag(passes_link)]
#[warning]
pub(crate) struct Link {
    #[label]
    pub span: Option<Span>,
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_no_link)]
pub(crate) struct NoLink {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub span: Span,
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_rustc_legacy_const_generics_only)]
pub(crate) struct RustcLegacyConstGenericsOnly {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub param_span: Span,
}
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_rustc_legacy_const_generics_index)]
pub(crate) struct RustcLegacyConstGenericsIndex {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub generics_span: Span,
}
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_rustc_legacy_const_generics_index_exceed)]
pub(crate) struct RustcLegacyConstGenericsIndexExceed {
    #[primary_span]
    #[label]
    pub span: Span,
    pub arg_count: usize,
}
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_rustc_legacy_const_generics_index_negative)]
pub(crate) struct RustcLegacyConstGenericsIndexNegative {
    #[primary_span]
    pub invalid_args: Vec<Span>,
}
```

## Block 54
**Metadata**: AST_ID=54 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_rustc_dirty_clean)]
pub(crate) struct RustcDirtyClean {
    #[primary_span]
    pub span: Span,
}
```

## Block 55
**Metadata**: AST_ID=55 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_repr_conflicting, code = E0566)]
pub(crate) struct ReprConflicting {
    #[primary_span]
    pub hint_spans: Vec<Span>,
}
```

## Block 56
**Metadata**: AST_ID=56 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_repr_align_greater_than_target_max, code = E0589)]
#[note]
pub(crate) struct InvalidReprAlignForTarget {
    #[primary_span]
    pub span: Span,
    pub size: u64,
}
```

## Block 57
**Metadata**: AST_ID=57 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=13

```rust
#[derive(LintDiagnostic)]
#[diag(passes_repr_conflicting, code = E0566)]
pub(crate) struct ReprConflictingLint;

#[derive(Diagnostic)]
#[diag(passes_macro_only_attribute)]
pub(crate) struct MacroOnlyAttribute {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub span: Span,
}
```

## Block 58
**Metadata**: AST_ID=58 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_debug_visualizer_placement)]
pub(crate) struct DebugVisualizerPlacement {
    #[primary_span]
    pub span: Span,
}
```

## Block 59
**Metadata**: AST_ID=59 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(passes_debug_visualizer_invalid)]
#[note(passes_note_1)]
#[note(passes_note_2)]
#[note(passes_note_3)]
pub(crate) struct DebugVisualizerInvalid {
    #[primary_span]
    pub span: Span,
}
```

## Block 60
**Metadata**: AST_ID=60 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_debug_visualizer_unreadable)]
pub(crate) struct DebugVisualizerUnreadable<'a> {
    #[primary_span]
    pub span: Span,
    pub file: &'a Path,
    pub error: Error,
}
```

## Block 61
**Metadata**: AST_ID=61 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_rustc_allow_const_fn_unstable)]
pub(crate) struct RustcAllowConstFnUnstable {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub span: Span,
}
```

## Block 62
**Metadata**: AST_ID=62 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_rustc_pub_transparent)]
pub(crate) struct RustcPubTransparent {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub span: Span,
}
```

## Block 63
**Metadata**: AST_ID=63 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_rustc_force_inline_coro)]
pub(crate) struct RustcForceInlineCoro {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub span: Span,
}
```

## Block 64
**Metadata**: AST_ID=64 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=16

```rust
#[derive(LintDiagnostic)]
pub(crate) enum MacroExport {
    #[diag(passes_macro_export)]
    Normal,

    #[diag(passes_macro_export_on_decl_macro)]
    #[note]
    OnDeclMacro,

    #[diag(passes_invalid_macro_export_arguments)]
    InvalidArgument,

    #[diag(passes_invalid_macro_export_arguments_too_many_items)]
    TooManyItems,
}
```

## Block 65
**Metadata**: AST_ID=65 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=12

```rust
#[derive(Subdiagnostic)]
pub(crate) enum UnusedNote {
    #[note(passes_unused_empty_lints_note)]
    EmptyList { name: Symbol },
    #[note(passes_unused_no_lints_note)]
    NoLints { name: Symbol },
    #[note(passes_unused_default_method_body_const_note)]
    DefaultMethodBodyConst,
    #[note(passes_unused_linker_messages_note)]
    LinkerMessagesBinaryCrateOnly,
}
```

## Block 66
**Metadata**: AST_ID=66 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(LintDiagnostic)]
#[diag(passes_unused)]
pub(crate) struct Unused {
    #[suggestion(code = "", applicability = "machine-applicable")]
    pub attr_span: Span,
    #[subdiagnostic]
    pub note: UnusedNote,
}
```

## Block 67
**Metadata**: AST_ID=67 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_non_exported_macro_invalid_attrs, code = E0518)]
pub(crate) struct NonExportedMacroInvalidAttrs {
    #[primary_span]
    #[label]
    pub attr_span: Span,
}
```

## Block 68
**Metadata**: AST_ID=68 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_may_dangle)]
pub(crate) struct InvalidMayDangle {
    #[primary_span]
    pub attr_span: Span,
}
```

## Block 69
**Metadata**: AST_ID=69 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(LintDiagnostic)]
#[diag(passes_unused_duplicate)]
pub(crate) struct UnusedDuplicate {
    #[suggestion(code = "", applicability = "machine-applicable")]
    pub this: Span,
    #[note]
    pub other: Span,
    #[warning]
    pub warning: bool,
}
```

## Block 70
**Metadata**: AST_ID=70 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Diagnostic)]
#[diag(passes_unused_multiple)]
pub(crate) struct UnusedMultiple {
    #[primary_span]
    #[suggestion(code = "", applicability = "machine-applicable")]
    pub this: Span,
    #[note]
    pub other: Span,
    pub name: Symbol,
}
```

## Block 71
**Metadata**: AST_ID=71 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_rustc_lint_opt_ty)]
pub(crate) struct RustcLintOptTy {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub span: Span,
}
```

## Block 72
**Metadata**: AST_ID=72 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_rustc_lint_opt_deny_field_access)]
pub(crate) struct RustcLintOptDenyFieldAccess {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub span: Span,
}
```

## Block 73
**Metadata**: AST_ID=73 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_collapse_debuginfo)]
pub(crate) struct CollapseDebuginfo {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub defn_span: Span,
}
```

## Block 74
**Metadata**: AST_ID=74 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(LintDiagnostic)]
#[diag(passes_deprecated_annotation_has_no_effect)]
pub(crate) struct DeprecatedAnnotationHasNoEffect {
    #[suggestion(applicability = "machine-applicable", code = "")]
    pub span: Span,
}
```

## Block 75
**Metadata**: AST_ID=75 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_unknown_external_lang_item, code = E0264)]
pub(crate) struct UnknownExternLangItem {
    #[primary_span]
    pub span: Span,
    pub lang_item: Symbol,
}
```

## Block 76
**Metadata**: AST_ID=76 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=18

```rust
#[derive(Diagnostic)]
#[diag(passes_missing_panic_handler)]
pub(crate) struct MissingPanicHandler;

#[derive(Diagnostic)]
#[diag(passes_panic_unwind_without_std)]
#[help]
#[note]
pub(crate) struct PanicUnwindWithoutStd;

#[derive(Diagnostic)]
#[diag(passes_missing_lang_item)]
#[note]
#[help]
pub(crate) struct MissingLangItem {
    pub name: Symbol,
}
```

## Block 77
**Metadata**: AST_ID=77 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(passes_lang_item_fn_with_track_caller)]
pub(crate) struct LangItemWithTrackCaller {
    #[primary_span]
    pub attr_span: Span,
    pub name: Symbol,
    #[label]
    pub sig_span: Span,
}
```

## Block 78
**Metadata**: AST_ID=78 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(passes_lang_item_fn_with_target_feature)]
pub(crate) struct LangItemWithTargetFeature {
    #[primary_span]
    pub attr_span: Span,
    pub name: Symbol,
    #[label]
    pub sig_span: Span,
}
```

## Block 79
**Metadata**: AST_ID=79 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Diagnostic)]
#[diag(passes_lang_item_on_incorrect_target, code = E0718)]
pub(crate) struct LangItemOnIncorrectTarget {
    #[primary_span]
    #[label]
    pub span: Span,
    pub name: Symbol,
    pub expected_target: Target,
    pub actual_target: Target,
}
```

## Block 80
**Metadata**: AST_ID=80 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_unknown_lang_item, code = E0522)]
pub(crate) struct UnknownLangItem {
    #[primary_span]
    #[label]
    pub span: Span,
    pub name: Symbol,
}
```

## Block 81
**Metadata**: AST_ID=81 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
pub(crate) struct InvalidAttrAtCrateLevel {
    pub span: Span,
    pub sugg_span: Option<Span>,
    pub name: Symbol,
    pub item: Option<ItemFollowingInnerAttr>,
}
```

## Block 82
**Metadata**: AST_ID=82 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, Copy)]
pub(crate) struct ItemFollowingInnerAttr {
    pub span: Span,
    pub kind: &'static str,
}
```

## Block 83
**Metadata**: AST_ID=83 | TYPE=FUNCTION | NAME=into_diag | COMPLEXITY=14 | LINES=24

```rust
impl<G: EmissionGuarantee> Diagnostic<'_, G> for InvalidAttrAtCrateLevel {
    #[track_caller]
    fn into_diag(self, dcx: DiagCtxtHandle<'_>, level: Level) -> Diag<'_, G> {
        let mut diag = Diag::new(dcx, level, fluent::passes_invalid_attr_at_crate_level);
        diag.span(self.span);
        diag.arg("name", self.name);
        // Only emit an error with a suggestion if we can create a string out
        // of the attribute span
        if let Some(span) = self.sugg_span {
            diag.span_suggestion_verbose(
                span,
                fluent::passes_suggestion,
                String::new(),
                Applicability::MachineApplicable,
            );
        }
        if let Some(item) = self.item {
            diag.arg("kind", item.kind);
            diag.span_label(item.span, fluent::passes_invalid_attr_at_crate_level_item);
        }
        diag
    }
}
```

## Block 84
**Metadata**: AST_ID=84 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=14

```rust
#[derive(Diagnostic)]
#[diag(passes_duplicate_diagnostic_item_in_crate)]
pub(crate) struct DuplicateDiagnosticItemInCrate {
    #[primary_span]
    pub duplicate_span: Option<Span>,
    #[note(passes_diagnostic_item_first_defined)]
    pub orig_span: Option<Span>,
    #[note]
    pub different_crates: bool,
    pub crate_name: Symbol,
    pub orig_crate_name: Symbol,
    pub name: Symbol,
}
```

## Block 85
**Metadata**: AST_ID=85 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_layout_abi)]
pub(crate) struct LayoutAbi {
    #[primary_span]
    pub span: Span,
    pub abi: String,
}
```

## Block 86
**Metadata**: AST_ID=86 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_layout_align)]
pub(crate) struct LayoutAlign {
    #[primary_span]
    pub span: Span,
    pub align: String,
}
```

## Block 87
**Metadata**: AST_ID=87 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_layout_size)]
pub(crate) struct LayoutSize {
    #[primary_span]
    pub span: Span,
    pub size: String,
}
```

## Block 88
**Metadata**: AST_ID=88 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_layout_homogeneous_aggregate)]
pub(crate) struct LayoutHomogeneousAggregate {
    #[primary_span]
    pub span: Span,
    pub homogeneous_aggregate: String,
}
```

## Block 89
**Metadata**: AST_ID=89 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_layout_of)]
pub(crate) struct LayoutOf<'tcx> {
    #[primary_span]
    pub span: Span,
    pub normalized_ty: Ty<'tcx>,
    pub ty_layout: String,
}
```

## Block 90
**Metadata**: AST_ID=90 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_layout_invalid_attribute)]
pub(crate) struct LayoutInvalidAttribute {
    #[primary_span]
    pub span: Span,
}
```

## Block 91
**Metadata**: AST_ID=91 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_abi_of)]
pub(crate) struct AbiOf {
    #[primary_span]
    pub span: Span,
    pub fn_name: Symbol,
    pub fn_abi: String,
}
```

## Block 92
**Metadata**: AST_ID=92 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_abi_ne)]
pub(crate) struct AbiNe {
    #[primary_span]
    pub span: Span,
    pub left: String,
    pub right: String,
}
```

## Block 93
**Metadata**: AST_ID=93 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_abi_invalid_attribute)]
pub(crate) struct AbiInvalidAttribute {
    #[primary_span]
    pub span: Span,
}
```

## Block 94
**Metadata**: AST_ID=94 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_unrecognized_argument)]
pub(crate) struct UnrecognizedArgument {
    #[primary_span]
    pub span: Span,
}
```

## Block 95
**Metadata**: AST_ID=95 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(passes_feature_stable_twice, code = E0711)]
pub(crate) struct FeatureStableTwice {
    #[primary_span]
    pub span: Span,
    pub feature: Symbol,
    pub since: Symbol,
    pub prev_since: Symbol,
}
```

## Block 96
**Metadata**: AST_ID=96 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(passes_feature_previously_declared, code = E0711)]
pub(crate) struct FeaturePreviouslyDeclared<'a> {
    #[primary_span]
    pub span: Span,
    pub feature: Symbol,
    pub declared: &'a str,
    pub prev_declared: &'a str,
}
```

## Block 97
**Metadata**: AST_ID=97 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_attr_only_in_functions)]
pub(crate) struct AttrOnlyInFunctions {
    #[primary_span]
    pub span: Span,
    pub attr: Symbol,
}
```

## Block 98
**Metadata**: AST_ID=98 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Diagnostic)]
#[diag(passes_multiple_rustc_main, code = E0137)]
pub(crate) struct MultipleRustcMain {
    #[primary_span]
    pub span: Span,
    #[label(passes_first)]
    pub first: Span,
    #[label(passes_additional)]
    pub additional: Span,
}
```

## Block 99
**Metadata**: AST_ID=99 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_extern_main)]
pub(crate) struct ExternMain {
    #[primary_span]
    pub span: Span,
}
```

## Block 100
**Metadata**: AST_ID=100 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
pub(crate) struct NoMainErr {
    pub sp: Span,
    pub crate_name: Symbol,
    pub has_filename: bool,
    pub filename: PathBuf,
    pub file_empty: bool,
    pub non_main_fns: Vec<Span>,
    pub main_def_opt: Option<MainDefinition>,
    pub add_teach_note: bool,
}
```

## Block 101
**Metadata**: AST_ID=101 | TYPE=FUNCTION | NAME=into_diag | COMPLEXITY=28 | LINES=43

```rust
impl<'a, G: EmissionGuarantee> Diagnostic<'a, G> for NoMainErr {
    #[track_caller]
    fn into_diag(self, dcx: DiagCtxtHandle<'a>, level: Level) -> Diag<'a, G> {
        let mut diag = Diag::new(dcx, level, fluent::passes_no_main_function);
        diag.span(DUMMY_SP);
        diag.code(E0601);
        diag.arg("crate_name", self.crate_name);
        diag.arg("filename", self.filename);
        diag.arg("has_filename", self.has_filename);
        let note = if !self.non_main_fns.is_empty() {
            for &span in &self.non_main_fns {
                diag.span_note(span, fluent::passes_here_is_main);
            }
            diag.note(fluent::passes_one_or_more_possible_main);
            diag.help(fluent::passes_consider_moving_main);
            // There were some functions named `main` though. Try to give the user a hint.
            fluent::passes_main_must_be_defined_at_crate
        } else if self.has_filename {
            fluent::passes_consider_adding_main_to_file
        } else {
            fluent::passes_consider_adding_main_at_crate
        };
        if self.file_empty {
            diag.note(note);
        } else {
            diag.span(self.sp.shrink_to_hi());
            diag.span_label(self.sp.shrink_to_hi(), note);
        }

        if let Some(main_def) = self.main_def_opt
            && main_def.opt_fn_def_id().is_none()
        {
            // There is something at `crate::main`, but it is not a function definition.
            diag.span_label(main_def.span, fluent::passes_non_function_main);
        }

        if self.add_teach_note {
            diag.note(fluent::passes_teach_note);
        }
        diag
    }
}
```

## Block 102
**Metadata**: AST_ID=102 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=15

```rust
pub(crate) struct DuplicateLangItem {
    pub local_span: Option<Span>,
    pub lang_item_name: Symbol,
    pub crate_name: Symbol,
    pub dependency_of: Option<Symbol>,
    pub is_local: bool,
    pub path: String,
    pub first_defined_span: Option<Span>,
    pub orig_crate_name: Option<Symbol>,
    pub orig_dependency_of: Option<Symbol>,
    pub orig_is_local: bool,
    pub orig_path: String,
    pub(crate) duplicate: Duplicate,
}
```

## Block 103
**Metadata**: AST_ID=103 | TYPE=FUNCTION | NAME=into_diag | COMPLEXITY=41 | LINES=54

```rust
impl<G: EmissionGuarantee> Diagnostic<'_, G> for DuplicateLangItem {
    #[track_caller]
    fn into_diag(self, dcx: DiagCtxtHandle<'_>, level: Level) -> Diag<'_, G> {
        let mut diag = Diag::new(
            dcx,
            level,
            match self.duplicate {
                Duplicate::Plain => fluent::passes_duplicate_lang_item,
                Duplicate::Crate => fluent::passes_duplicate_lang_item_crate,
                Duplicate::CrateDepends => fluent::passes_duplicate_lang_item_crate_depends,
            },
        );
        diag.code(E0152);
        diag.arg("lang_item_name", self.lang_item_name);
        diag.arg("crate_name", self.crate_name);
        if let Some(dependency_of) = self.dependency_of {
            diag.arg("dependency_of", dependency_of);
        }
        diag.arg("path", self.path);
        if let Some(orig_crate_name) = self.orig_crate_name {
            diag.arg("orig_crate_name", orig_crate_name);
        }
        if let Some(orig_dependency_of) = self.orig_dependency_of {
            diag.arg("orig_dependency_of", orig_dependency_of);
        }
        diag.arg("orig_path", self.orig_path);
        if let Some(span) = self.local_span {
            diag.span(span);
        }
        if let Some(span) = self.first_defined_span {
            diag.span_note(span, fluent::passes_first_defined_span);
        } else {
            if self.orig_dependency_of.is_none() {
                diag.note(fluent::passes_first_defined_crate);
            } else {
                diag.note(fluent::passes_first_defined_crate_depends);
            }

            if self.orig_is_local {
                diag.note(fluent::passes_first_definition_local);
            } else {
                diag.note(fluent::passes_first_definition_path);
            }

            if self.is_local {
                diag.note(fluent::passes_second_definition_local);
            } else {
                diag.note(fluent::passes_second_definition_path);
            }
        }
        diag
    }
}
```

## Block 104
**Metadata**: AST_ID=104 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=14

```rust
#[derive(Diagnostic)]
#[diag(passes_incorrect_target, code = E0718)]
pub(crate) struct IncorrectTarget<'a> {
    #[primary_span]
    pub span: Span,
    #[label]
    pub generics_span: Span,
    pub name: &'a str, // cannot be symbol because it renders e.g. `r#fn` instead of `fn`
    pub kind: &'static str,
    pub num: usize,
    pub actual_num: usize,
    pub at_least: bool,
}
```

## Block 105
**Metadata**: AST_ID=105 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_incorrect_crate_type)]
pub(crate) struct IncorrectCrateType {
    #[primary_span]
    pub span: Span,
}
```

## Block 106
**Metadata**: AST_ID=106 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(LintDiagnostic)]
#[diag(passes_useless_assignment)]
pub(crate) struct UselessAssignment<'a> {
    pub is_field_assign: bool,
    pub ty: Ty<'a>,
}
```

## Block 107
**Metadata**: AST_ID=107 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
#[derive(LintDiagnostic)]
#[diag(passes_inline_ignored_for_exported)]
#[help]
pub(crate) struct InlineIgnoredForExported {}
```

## Block 108
**Metadata**: AST_ID=108 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_object_lifetime_err)]
pub(crate) struct ObjectLifetimeErr {
    #[primary_span]
    pub span: Span,
    pub repr: String,
}
```

## Block 109
**Metadata**: AST_ID=109 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=7 | LINES=32

```rust
#[derive(Diagnostic)]
pub(crate) enum AttrApplication {
    #[diag(passes_attr_application_enum, code = E0517)]
    Enum {
        #[primary_span]
        hint_span: Span,
        #[label]
        span: Span,
    },
    #[diag(passes_attr_application_struct, code = E0517)]
    Struct {
        #[primary_span]
        hint_span: Span,
        #[label]
        span: Span,
    },
    #[diag(passes_attr_application_struct_union, code = E0517)]
    StructUnion {
        #[primary_span]
        hint_span: Span,
        #[label]
        span: Span,
    },
    #[diag(passes_attr_application_struct_enum_union, code = E0517)]
    StructEnumUnion {
        #[primary_span]
        hint_span: Span,
        #[label]
        span: Span,
    },
}
```

## Block 110
**Metadata**: AST_ID=110 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_transparent_incompatible, code = E0692)]
pub(crate) struct TransparentIncompatible {
    #[primary_span]
    pub hint_spans: Vec<Span>,
    pub target: String,
}
```

## Block 111
**Metadata**: AST_ID=111 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(passes_deprecated_attribute, code = E0549)]
pub(crate) struct DeprecatedAttribute {
    #[primary_span]
    pub span: Span,
}
```

## Block 112
**Metadata**: AST_ID=112 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(passes_useless_stability)]
pub(crate) struct UselessStability {
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(passes_item)]
    pub item_sp: Span,
}
```

## Block 113
**Metadata**: AST_ID=113 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(passes_cannot_stabilize_deprecated)]
pub(crate) struct CannotStabilizeDeprecated {
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(passes_item)]
    pub item_sp: Span,
}
```

## Block 114
**Metadata**: AST_ID=114 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Diagnostic)]
#[diag(passes_unstable_attr_for_already_stable_feature)]
pub(crate) struct UnstableAttrForAlreadyStableFeature {
    #[primary_span]
    #[label]
    #[help]
    pub attr_span: Span,
    #[label(passes_item)]
    pub item_span: Span,
}
```

## Block 115
**Metadata**: AST_ID=115 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_missing_stability_attr)]
pub(crate) struct MissingStabilityAttr<'a> {
    #[primary_span]
    pub span: Span,
    pub descr: &'a str,
}
```

## Block 116
**Metadata**: AST_ID=116 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_missing_const_stab_attr)]
pub(crate) struct MissingConstStabAttr<'a> {
    #[primary_span]
    pub span: Span,
    pub descr: &'a str,
}
```

## Block 117
**Metadata**: AST_ID=117 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_trait_impl_const_stable)]
#[note]
pub(crate) struct TraitImplConstStable {
    #[primary_span]
    pub span: Span,
}
```

## Block 118
**Metadata**: AST_ID=118 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=5 | LINES=11

```rust
#[derive(Diagnostic)]
#[diag(passes_trait_impl_const_stability_mismatch)]
pub(crate) struct TraitImplConstStabilityMismatch {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub impl_stability: ImplConstStability,
    #[subdiagnostic]
    pub trait_stability: TraitConstStability,
}
```

## Block 119
**Metadata**: AST_ID=119 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=14

```rust
#[derive(Subdiagnostic)]
pub(crate) enum TraitConstStability {
    #[note(passes_trait_impl_const_stability_mismatch_trait_stable)]
    Stable {
        #[primary_span]
        span: Span,
    },
    #[note(passes_trait_impl_const_stability_mismatch_trait_unstable)]
    Unstable {
        #[primary_span]
        span: Span,
    },
}
```

## Block 120
**Metadata**: AST_ID=120 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=14

```rust
#[derive(Subdiagnostic)]
pub(crate) enum ImplConstStability {
    #[note(passes_trait_impl_const_stability_mismatch_impl_stable)]
    Stable {
        #[primary_span]
        span: Span,
    },
    #[note(passes_trait_impl_const_stability_mismatch_impl_unstable)]
    Unstable {
        #[primary_span]
        span: Span,
    },
}
```

## Block 121
**Metadata**: AST_ID=121 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_unknown_feature, code = E0635)]
pub(crate) struct UnknownFeature {
    #[primary_span]
    pub span: Span,
    pub feature: Symbol,
}
```

## Block 122
**Metadata**: AST_ID=122 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_unknown_feature_alias, code = E0635)]
pub(crate) struct RenamedFeature {
    #[primary_span]
    pub span: Span,
    pub feature: Symbol,
    pub alias: Symbol,
}
```

## Block 123
**Metadata**: AST_ID=123 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_implied_feature_not_exist)]
pub(crate) struct ImpliedFeatureNotExist {
    #[primary_span]
    pub span: Span,
    pub feature: Symbol,
    pub implied_by: Symbol,
}
```

## Block 124
**Metadata**: AST_ID=124 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_duplicate_feature_err, code = E0636)]
pub(crate) struct DuplicateFeatureErr {
    #[primary_span]
    pub span: Span,
    pub feature: Symbol,
}
```

## Block 125
**Metadata**: AST_ID=125 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_missing_const_err)]
pub(crate) struct MissingConstErr {
    #[primary_span]
    #[help]
    pub fn_sig_span: Span,
}
```

## Block 126
**Metadata**: AST_ID=126 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_const_stable_not_stable)]
pub(crate) struct ConstStableNotStable {
    #[primary_span]
    pub fn_sig_span: Span,
    #[label]
    pub const_span: Span,
}
```

## Block 127
**Metadata**: AST_ID=127 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=8 | LINES=33

```rust
#[derive(LintDiagnostic)]
pub(crate) enum MultipleDeadCodes<'tcx> {
    #[diag(passes_dead_codes)]
    DeadCodes {
        multiple: bool,
        num: usize,
        descr: &'tcx str,
        participle: &'tcx str,
        name_list: DiagSymbolList,
        #[subdiagnostic]
        // only on DeadCodes since it's never a problem for tuple struct fields
        enum_variants_with_same_name: Vec<EnumVariantSameName<'tcx>>,
        #[subdiagnostic]
        parent_info: Option<ParentInfo<'tcx>>,
        #[subdiagnostic]
        ignored_derived_impls: Option<IgnoredDerivedImpls>,
    },
    #[diag(passes_dead_codes)]
    UnusedTupleStructFields {
        multiple: bool,
        num: usize,
        descr: &'tcx str,
        participle: &'tcx str,
        name_list: DiagSymbolList,
        #[subdiagnostic]
        change_fields_suggestion: ChangeFields,
        #[subdiagnostic]
        parent_info: Option<ParentInfo<'tcx>>,
        #[subdiagnostic]
        ignored_derived_impls: Option<IgnoredDerivedImpls>,
    },
}
```

## Block 128
**Metadata**: AST_ID=128 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Subdiagnostic)]
#[note(passes_enum_variant_same_name)]
pub(crate) struct EnumVariantSameName<'tcx> {
    #[primary_span]
    pub variant_span: Span,
    pub dead_name: Symbol,
    pub dead_descr: &'tcx str,
}
```

## Block 129
**Metadata**: AST_ID=129 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Subdiagnostic)]
#[label(passes_parent_info)]
pub(crate) struct ParentInfo<'tcx> {
    pub num: usize,
    pub descr: &'tcx str,
    pub parent_descr: &'tcx str,
    #[primary_span]
    pub span: Span,
}
```

## Block 130
**Metadata**: AST_ID=130 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Subdiagnostic)]
#[note(passes_ignored_derived_impls)]
pub(crate) struct IgnoredDerivedImpls {
    pub name: Symbol,
    pub trait_list: DiagSymbolList,
    pub trait_list_len: usize,
}
```

## Block 131
**Metadata**: AST_ID=131 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=15

```rust
#[derive(Subdiagnostic)]
pub(crate) enum ChangeFields {
    #[multipart_suggestion(
        passes_change_fields_to_be_of_unit_type,
        applicability = "has-placeholders"
    )]
    ChangeToUnitTypeOrRemove {
        num: usize,
        #[suggestion_part(code = "()")]
        spans: Vec<Span>,
    },
    #[help(passes_remove_fields)]
    Remove { num: usize },
}
```

## Block 132
**Metadata**: AST_ID=132 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_proc_macro_bad_sig)]
pub(crate) struct ProcMacroBadSig {
    #[primary_span]
    pub span: Span,
    pub kind: ProcMacroKind,
}
```

## Block 133
**Metadata**: AST_ID=133 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
#[derive(LintDiagnostic)]
#[diag(passes_unreachable_due_to_uninhabited)]
pub(crate) struct UnreachableDueToUninhabited<'desc, 'tcx> {
    pub descr: &'desc str,
    #[label]
    pub expr: Span,
    #[label(passes_label_orig)]
    #[note]
    pub orig: Span,
    pub ty: Ty<'tcx>,
}
```

## Block 134
**Metadata**: AST_ID=134 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(LintDiagnostic)]
#[diag(passes_unused_var_maybe_capture_ref)]
#[help]
pub(crate) struct UnusedVarMaybeCaptureRef {
    pub name: String,
}
```

## Block 135
**Metadata**: AST_ID=135 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(LintDiagnostic)]
#[diag(passes_unused_capture_maybe_capture_ref)]
#[help]
pub(crate) struct UnusedCaptureMaybeCaptureRef {
    pub name: String,
}
```

## Block 136
**Metadata**: AST_ID=136 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(LintDiagnostic)]
#[diag(passes_unused_var_remove_field)]
pub(crate) struct UnusedVarRemoveField {
    pub name: String,
    #[subdiagnostic]
    pub sugg: UnusedVarRemoveFieldSugg,
}
```

## Block 137
**Metadata**: AST_ID=137 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Subdiagnostic)]
#[multipart_suggestion(
    passes_unused_var_remove_field_suggestion,
    applicability = "machine-applicable"
)]
pub(crate) struct UnusedVarRemoveFieldSugg {
    #[suggestion_part(code = "")]
    pub spans: Vec<Span>,
}
```

## Block 138
**Metadata**: AST_ID=138 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(LintDiagnostic)]
#[diag(passes_unused_var_assigned_only)]
#[note]
pub(crate) struct UnusedVarAssignedOnly {
    pub name: String,
    #[subdiagnostic]
    pub typo: Option<PatternTypo>,
}
```

## Block 139
**Metadata**: AST_ID=139 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=14

```rust
#[derive(Subdiagnostic)]
#[multipart_suggestion(
    passes_unused_var_typo,
    style = "verbose",
    applicability = "machine-applicable"
)]
pub(crate) struct PatternTypo {
    #[suggestion_part(code = "{code}")]
    pub span: Span,
    pub code: String,
    pub item_name: String,
    pub kind: String,
}
```

## Block 140
**Metadata**: AST_ID=140 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(LintDiagnostic)]
#[diag(passes_unnecessary_stable_feature)]
pub(crate) struct UnnecessaryStableFeature {
    pub feature: Symbol,
    pub since: Symbol,
}
```

## Block 141
**Metadata**: AST_ID=141 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=12

```rust
#[derive(LintDiagnostic)]
#[diag(passes_unnecessary_partial_stable_feature)]
pub(crate) struct UnnecessaryPartialStableFeature {
    #[suggestion(code = "{implies}", applicability = "maybe-incorrect")]
    pub span: Span,
    #[suggestion(passes_suggestion_remove, code = "", applicability = "maybe-incorrect")]
    pub line: Span,
    pub feature: Symbol,
    pub since: Symbol,
    pub implies: Symbol,
}
```

## Block 142
**Metadata**: AST_ID=142 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=15

```rust
#[derive(LintDiagnostic)]
#[diag(passes_ineffective_unstable_impl)]
#[note]
pub(crate) struct IneffectiveUnstableImpl;

#[derive(LintDiagnostic)]
#[diag(passes_unused_assign)]
pub(crate) struct UnusedAssign {
    pub name: String,
    #[subdiagnostic]
    pub suggestion: Option<UnusedAssignSuggestion>,
    #[help]
    pub help: bool,
}
```

## Block 143
**Metadata**: AST_ID=143 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=14

```rust
#[derive(Subdiagnostic)]
#[multipart_suggestion(passes_unused_assign_suggestion, applicability = "maybe-incorrect")]
pub(crate) struct UnusedAssignSuggestion {
    pub pre: &'static str,
    #[suggestion_part(code = "{pre}mut ")]
    pub ty_span: Option<Span>,
    #[suggestion_part(code = "")]
    pub ty_ref_span: Span,
    #[suggestion_part(code = "*")]
    pub ident_span: Span,
    #[suggestion_part(code = "")]
    pub expr_ref_span: Span,
}
```

## Block 144
**Metadata**: AST_ID=144 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(LintDiagnostic)]
#[diag(passes_unused_assign_passed)]
#[help]
pub(crate) struct UnusedAssignPassed {
    pub name: String,
}
```

## Block 145
**Metadata**: AST_ID=145 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=14

```rust
#[derive(LintDiagnostic)]
#[diag(passes_unused_variable_try_prefix)]
pub(crate) struct UnusedVariableTryPrefix {
    #[label]
    pub label: Option<Span>,
    #[subdiagnostic]
    pub string_interp: Vec<UnusedVariableStringInterp>,
    #[subdiagnostic]
    pub sugg: UnusedVariableSugg,
    pub name: String,
    #[subdiagnostic]
    pub typo: Option<PatternTypo>,
}
```

## Block 146
**Metadata**: AST_ID=146 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=16

```rust
#[derive(Subdiagnostic)]
pub(crate) enum UnusedVariableSugg {
    #[multipart_suggestion(passes_suggestion, applicability = "maybe-incorrect")]
    TryPrefixSugg {
        #[suggestion_part(code = "_{name}")]
        spans: Vec<Span>,
        name: String,
    },
    #[help(passes_unused_variable_args_in_macro)]
    NoSugg {
        #[primary_span]
        span: Span,
        name: String,
    },
}
```

## Block 147
**Metadata**: AST_ID=147 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
pub(crate) struct UnusedVariableStringInterp {
    pub lit: Span,
    pub lo: Span,
    pub hi: Span,
}
```

## Block 148
**Metadata**: AST_ID=148 | TYPE=FUNCTION | NAME=add_to_diag | COMPLEXITY=5 | LINES=11

```rust
impl Subdiagnostic for UnusedVariableStringInterp {
    fn add_to_diag<G: EmissionGuarantee>(self, diag: &mut Diag<'_, G>) {
        diag.span_label(self.lit, crate::fluent_generated::passes_maybe_string_interpolation);
        diag.multipart_suggestion(
            crate::fluent_generated::passes_string_interpolation_only_works,
            vec![(self.lo, String::from("format!(")), (self.hi, String::from(")"))],
            Applicability::MachineApplicable,
        );
    }
}
```

## Block 149
**Metadata**: AST_ID=149 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(LintDiagnostic)]
#[diag(passes_unused_variable_try_ignore)]
pub(crate) struct UnusedVarTryIgnore {
    pub name: String,
    #[subdiagnostic]
    pub sugg: UnusedVarTryIgnoreSugg,
}
```

## Block 150
**Metadata**: AST_ID=150 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=10

```rust
#[derive(Subdiagnostic)]
#[multipart_suggestion(passes_suggestion, applicability = "maybe-incorrect")]
pub(crate) struct UnusedVarTryIgnoreSugg {
    #[suggestion_part(code = "{name}: _")]
    pub shorthands: Vec<Span>,
    #[suggestion_part(code = "_")]
    pub non_shorthands: Vec<Span>,
    pub name: String,
}
```

## Block 151
**Metadata**: AST_ID=151 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(LintDiagnostic)]
#[diag(passes_attr_crate_level)]
#[note]
pub(crate) struct AttrCrateLevelOnly {
    #[subdiagnostic]
    pub sugg: Option<AttrCrateLevelOnlySugg>,
}
```

## Block 152
**Metadata**: AST_ID=152 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Subdiagnostic)]
#[suggestion(passes_suggestion, applicability = "maybe-incorrect", code = "!", style = "verbose")]
pub(crate) struct AttrCrateLevelOnlySugg {
    #[primary_span]
    pub attr: Span,
}
```

## Block 153
**Metadata**: AST_ID=153 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=17

```rust
/// "sanitize attribute not allowed here"
#[derive(Diagnostic)]
#[diag(passes_sanitize_attribute_not_allowed)]
pub(crate) struct SanitizeAttributeNotAllowed {
    #[primary_span]
    pub attr_span: Span,
    /// "not a function, impl block, or module"
    #[label(passes_not_fn_impl_mod)]
    pub not_fn_impl_mod: Option<Span>,
    /// "function has no body"
    #[label(passes_no_body)]
    pub no_body: Option<Span>,
    /// "sanitize attribute can be applied to a function (with body), impl block, or module"
    #[help]
    pub help: (),
}
```

## Block 154
**Metadata**: AST_ID=154 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
// FIXME(jdonszelmann): move back to rustc_attr
#[derive(Diagnostic)]
#[diag(passes_rustc_const_stable_indirect_pairing)]
pub(crate) struct RustcConstStableIndirectPairing {
    #[primary_span]
    pub span: Span,
}
```

## Block 155
**Metadata**: AST_ID=155 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(passes_unsupported_attributes_in_where)]
#[help]
pub(crate) struct UnsupportedAttributesInWhere {
    #[primary_span]
    pub span: MultiSpan,
}
```

## Block 156
**Metadata**: AST_ID=156 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=8 | LINES=47

```rust
#[derive(Diagnostic)]
pub(crate) enum UnexportableItem<'a> {
    #[diag(passes_unexportable_item)]
    Item {
        #[primary_span]
        span: Span,
        descr: &'a str,
    },

    #[diag(passes_unexportable_generic_fn)]
    GenericFn(#[primary_span] Span),

    #[diag(passes_unexportable_fn_abi)]
    FnAbi(#[primary_span] Span),

    #[diag(passes_unexportable_type_repr)]
    TypeRepr(#[primary_span] Span),

    #[diag(passes_unexportable_type_in_interface)]
    TypeInInterface {
        #[primary_span]
        span: Span,
        desc: &'a str,
        ty: &'a str,
        #[label]
        ty_span: Span,
    },

    #[diag(passes_unexportable_priv_item)]
    PrivItem {
        #[primary_span]
        span: Span,
        #[note]
        vis_note: Span,
        vis_descr: &'a str,
    },

    #[diag(passes_unexportable_adt_with_private_fields)]
    AdtWithPrivFields {
        #[primary_span]
        span: Span,
        #[note]
        vis_note: Span,
        field_name: &'a str,
    },
}
```

## Block 157
**Metadata**: AST_ID=157 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_repr_align_should_be_align)]
pub(crate) struct ReprAlignShouldBeAlign {
    #[primary_span]
    #[help]
    pub span: Span,
    pub item: &'static str,
}
```

## Block 158
**Metadata**: AST_ID=158 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_repr_align_should_be_align_static)]
pub(crate) struct ReprAlignShouldBeAlignStatic {
    #[primary_span]
    #[help]
    pub span: Span,
    pub item: &'static str,
}
```

## Block 159
**Metadata**: AST_ID=159 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(passes_custom_mir_phase_requires_dialect)]
pub(crate) struct CustomMirPhaseRequiresDialect {
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub phase_span: Span,
}
```

## Block 160
**Metadata**: AST_ID=160 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=13

```rust
#[derive(Diagnostic)]
#[diag(passes_custom_mir_incompatible_dialect_and_phase)]
pub(crate) struct CustomMirIncompatibleDialectAndPhase {
    pub dialect: MirDialect,
    pub phase: MirPhase,
    #[primary_span]
    pub attr_span: Span,
    #[label]
    pub dialect_span: Span,
    #[label]
    pub phase_span: Span,
}
```

---
*Generated by AST tracing system*
