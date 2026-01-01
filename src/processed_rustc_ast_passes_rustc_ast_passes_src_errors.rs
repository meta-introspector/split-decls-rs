// SRC: ../rust/compiler/rustc_ast_passes/src/errors.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */
// Errors emitted by ast_passes.

use crate::rustc_abi::ExternAbi;
use crate::rustc_complete::ParamKindOrd;
use crate::rustc_complete::codes::*;
use crate::rustc_complete::{Applicability, Diag, EmissionGuarantee, Subdiagnostic};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use rustc_macros::{Diagnostic, LintDiagnostic, Subdiagnostic};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::{Ident, Span, Symbol};
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=17 */

use crate::fluent_generated as fluent;

#[derive(Diagnostic)]
#[diag(ast_passes_visibility_not_permitted, code = E0449)]
pub(crate) struct VisibilityNotPermitted {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub note: VisibilityNotPermittedNote,
    #[suggestion(
        ast_passes_remove_qualifier_sugg,
        code = "",
        applicability = "machine-applicable"
    )]
    pub remove_qualifier_sugg: Span,
}
/* AST_META: AST_ID=5 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=12 */

#[derive(Subdiagnostic)]
pub(crate) enum VisibilityNotPermittedNote {
    #[note(ast_passes_enum_variant)]
    EnumVariant,
    #[note(ast_passes_trait_impl)]
    TraitImpl,
    #[note(ast_passes_individual_impl_items)]
    IndividualImplItems,
    #[note(ast_passes_individual_foreign_items)]
    IndividualForeignItems,
}
/* AST_META: AST_ID=6 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=26 */

#[derive(Diagnostic)]
#[diag(ast_passes_trait_fn_const, code = E0379)]
pub(crate) struct TraitFnConst {
    #[primary_span]
    #[label]
    pub span: Span,
    pub in_impl: bool,
    #[label(ast_passes_const_context_label)]
    pub const_context_label: Option<Span>,
    #[suggestion(ast_passes_remove_const_sugg, code = "")]
    pub remove_const_sugg: (Span, Applicability),
    pub requires_multiple_changes: bool,
    #[suggestion(
        ast_passes_make_impl_const_sugg,
        code = "const ",
        applicability = "maybe-incorrect"
    )]
    pub make_impl_const_sugg: Option<Span>,
    #[suggestion(
        ast_passes_make_trait_const_sugg,
        code = "#[const_trait]\n",
        applicability = "maybe-incorrect"
    )]
    pub make_trait_const_sugg: Option<Span>,
}
/* AST_META: AST_ID=7 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(ast_passes_async_fn_in_const_trait_or_trait_impl)]
pub(crate) struct AsyncFnInConstTraitOrTraitImpl {
    #[primary_span]
    pub async_keyword: Span,
    pub in_impl: bool,
    #[label]
    pub const_keyword: Span,
}
/* AST_META: AST_ID=8 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_forbidden_bound)]
pub(crate) struct ForbiddenBound {
    #[primary_span]
    pub spans: Vec<Span>,
}
/* AST_META: AST_ID=9 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_forbidden_const_param)]
pub(crate) struct ForbiddenConstParam {
    #[primary_span]
    pub const_param_spans: Vec<Span>,
}
/* AST_META: AST_ID=10 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(ast_passes_fn_param_too_many)]
pub(crate) struct FnParamTooMany {
    #[primary_span]
    pub span: Span,
    pub max_num_args: usize,
}
/* AST_META: AST_ID=11 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_fn_param_c_var_args_not_last)]
pub(crate) struct FnParamCVarArgsNotLast {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=12 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(ast_passes_fn_param_doc_comment)]
pub(crate) struct FnParamDocComment {
    #[primary_span]
    #[label]
    pub span: Span,
}
/* AST_META: AST_ID=13 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_fn_param_forbidden_attr)]
pub(crate) struct FnParamForbiddenAttr {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=14 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_fn_param_forbidden_self)]
#[note]
pub(crate) struct FnParamForbiddenSelf {
    #[primary_span]
    #[label]
    pub span: Span,
}
/* AST_META: AST_ID=15 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_forbidden_default)]
pub(crate) struct ForbiddenDefault {
    #[primary_span]
    pub span: Span,
    #[label]
    pub def_span: Span,
}
/* AST_META: AST_ID=16 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_assoc_const_without_body)]
pub(crate) struct AssocConstWithoutBody {
    #[primary_span]
    pub span: Span,
    #[suggestion(code = " = <expr>;", applicability = "has-placeholders")]
    pub replace_span: Span,
}
/* AST_META: AST_ID=17 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_assoc_fn_without_body)]
pub(crate) struct AssocFnWithoutBody {
    #[primary_span]
    pub span: Span,
    #[suggestion(code = " {{ <body> }}", applicability = "has-placeholders")]
    pub replace_span: Span,
}
/* AST_META: AST_ID=18 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_assoc_type_without_body)]
pub(crate) struct AssocTypeWithoutBody {
    #[primary_span]
    pub span: Span,
    #[suggestion(code = " = <type>;", applicability = "has-placeholders")]
    pub replace_span: Span,
}
/* AST_META: AST_ID=19 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_const_without_body)]
pub(crate) struct ConstWithoutBody {
    #[primary_span]
    pub span: Span,
    #[suggestion(code = " = <expr>;", applicability = "has-placeholders")]
    pub replace_span: Span,
}
/* AST_META: AST_ID=20 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_static_without_body)]
pub(crate) struct StaticWithoutBody {
    #[primary_span]
    pub span: Span,
    #[suggestion(code = " = <expr>;", applicability = "has-placeholders")]
    pub replace_span: Span,
}
/* AST_META: AST_ID=21 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_ty_alias_without_body)]
pub(crate) struct TyAliasWithoutBody {
    #[primary_span]
    pub span: Span,
    #[suggestion(code = " = <type>;", applicability = "has-placeholders")]
    pub replace_span: Span,
}
/* AST_META: AST_ID=22 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=11 */

#[derive(Diagnostic)]
#[diag(ast_passes_fn_without_body)]
pub(crate) struct FnWithoutBody {
    #[primary_span]
    pub span: Span,
    #[suggestion(code = " {{ <body> }}", applicability = "has-placeholders")]
    pub replace_span: Span,
    #[subdiagnostic]
    pub extern_block_suggestion: Option<ExternBlockSuggestion>,
}
/* AST_META: AST_ID=23 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=10 | LINES=19 */

#[derive(Subdiagnostic)]
pub(crate) enum ExternBlockSuggestion {
    #[multipart_suggestion(ast_passes_extern_block_suggestion, applicability = "maybe-incorrect")]
    Implicit {
        #[suggestion_part(code = "extern {{")]
        start_span: Span,
        #[suggestion_part(code = " }}")]
        end_span: Span,
    },
    #[multipart_suggestion(ast_passes_extern_block_suggestion, applicability = "maybe-incorrect")]
    Explicit {
        #[suggestion_part(code = "extern \"{abi}\" {{")]
        start_span: Span,
        #[suggestion_part(code = " }}")]
        end_span: Span,
        abi: Symbol,
    },
}
/* AST_META: AST_ID=24 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=6 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_extern_invalid_safety)]
pub(crate) struct InvalidSafetyOnExtern {
    #[primary_span]
    pub item_span: Span,
    #[suggestion(code = "unsafe ", applicability = "machine-applicable", style = "verbose")]
    pub block: Option<Span>,
}
/* AST_META: AST_ID=25 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_item_invalid_safety)]
pub(crate) struct InvalidSafetyOnItem {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=26 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_fn_ptr_invalid_safety)]
pub(crate) struct InvalidSafetyOnFnPtr {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=27 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_unsafe_static)]
pub(crate) struct UnsafeStatic {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=28 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(ast_passes_bound_in_context)]
pub(crate) struct BoundInContext<'a> {
    #[primary_span]
    pub span: Span,
    pub ctx: &'a str,
}
/* AST_META: AST_ID=29 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=13 */

#[derive(Diagnostic)]
#[diag(ast_passes_extern_types_cannot)]
#[note(ast_passes_extern_keyword_link)]
pub(crate) struct ExternTypesCannotHave<'a> {
    #[primary_span]
    #[suggestion(code = "", applicability = "maybe-incorrect")]
    pub span: Span,
    pub descr: &'a str,
    pub remove_descr: &'a str,
    #[label]
    pub block_span: Span,
}
/* AST_META: AST_ID=30 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=14 */

#[derive(Diagnostic)]
#[diag(ast_passes_body_in_extern)]
#[note(ast_passes_extern_keyword_link)]
pub(crate) struct BodyInExtern<'a> {
    #[primary_span]
    #[label(ast_passes_cannot_have)]
    pub span: Span,
    #[label(ast_passes_invalid)]
    pub body: Span,
    #[label(ast_passes_existing)]
    pub block: Span,
    pub kind: &'a str,
}
/* AST_META: AST_ID=31 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=14 */

#[derive(Diagnostic)]
#[diag(ast_passes_fn_body_extern)]
#[help]
#[note(ast_passes_extern_keyword_link)]
pub(crate) struct FnBodyInExtern {
    #[primary_span]
    #[label(ast_passes_cannot_have)]
    pub span: Span,
    #[suggestion(code = ";", applicability = "maybe-incorrect")]
    pub body: Span,
    #[label]
    pub block: Span,
}
/* AST_META: AST_ID=32 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */

#[derive(Diagnostic)]
#[diag(ast_passes_extern_fn_qualifiers)]
pub(crate) struct FnQualifierInExtern {
    #[primary_span]
    #[suggestion(code = "", applicability = "maybe-incorrect")]
    pub span: Span,
    #[label]
    pub block: Span,
    pub kw: &'static str,
}
/* AST_META: AST_ID=33 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(ast_passes_extern_item_ascii)]
#[note]
pub(crate) struct ExternItemAscii {
    #[primary_span]
    pub span: Span,
    #[label]
    pub block: Span,
}
/* AST_META: AST_ID=34 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_c_variadic_associated_function)]
pub(crate) struct CVariadicAssociatedFunction {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=35 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(ast_passes_c_variadic_no_extern)]
#[help]
pub(crate) struct CVariadicNoExtern {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=36 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=6 | LINES=15 */

#[derive(Diagnostic)]
#[diag(ast_passes_c_variadic_must_be_unsafe)]
pub(crate) struct CVariadicMustBeUnsafe {
    #[primary_span]
    pub span: Span,

    #[suggestion(
        ast_passes_suggestion,
        applicability = "maybe-incorrect",
        code = "unsafe ",
        style = "verbose"
    )]
    pub unsafe_span: Span,
}
/* AST_META: AST_ID=37 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */

#[derive(Diagnostic)]
#[diag(ast_passes_c_variadic_bad_extern)]
#[help]
pub(crate) struct CVariadicBadExtern {
    #[primary_span]
    pub span: Span,
    pub abi: Symbol,
    #[label]
    pub extern_span: Span,
}
/* AST_META: AST_ID=38 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_item_underscore)]
pub(crate) struct ItemUnderscore<'a> {
    #[primary_span]
    #[label]
    pub span: Span,
    pub kind: &'a str,
}
/* AST_META: AST_ID=39 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_nomangle_ascii, code = E0754)]
pub(crate) struct NoMangleAscii {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=40 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_module_nonascii, code = E0754)]
#[help]
pub(crate) struct ModuleNonAscii {
    #[primary_span]
    pub span: Span,
    pub name: Symbol,
}
/* AST_META: AST_ID=41 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(ast_passes_auto_generic, code = E0567)]
pub(crate) struct AutoTraitGeneric {
    #[primary_span]
    #[suggestion(code = "", applicability = "machine-applicable", style = "tool-only")]
    pub span: Span,
    #[label]
    pub ident: Span,
}
/* AST_META: AST_ID=42 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */

#[derive(Diagnostic)]
#[diag(ast_passes_auto_super_lifetime, code = E0568)]
pub(crate) struct AutoTraitBounds {
    #[primary_span]
    pub span: Vec<Span>,
    #[suggestion(code = "", applicability = "machine-applicable", style = "tool-only")]
    pub removal: Span,
    #[label]
    pub ident: Span,
}
/* AST_META: AST_ID=43 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */

#[derive(Diagnostic)]
#[diag(ast_passes_auto_items, code = E0380)]
pub(crate) struct AutoTraitItems {
    #[primary_span]
    pub spans: Vec<Span>,
    #[suggestion(code = "", applicability = "machine-applicable", style = "tool-only")]
    pub total: Span,
    #[label]
    pub ident: Span,
}
/* AST_META: AST_ID=44 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=20 */

#[derive(Diagnostic)]
#[diag(ast_passes_generic_before_constraints)]
pub(crate) struct ArgsBeforeConstraint {
    #[primary_span]
    pub arg_spans: Vec<Span>,
    #[label(ast_passes_constraints)]
    pub constraints: Span,
    #[label(ast_passes_args)]
    pub args: Span,
    #[suggestion(code = "{suggestion}", applicability = "machine-applicable", style = "verbose")]
    pub data: Span,
    pub suggestion: String,
    pub constraint_len: usize,
    pub args_len: usize,
    #[subdiagnostic]
    pub constraint_spans: EmptyLabelManySpans,
    #[subdiagnostic]
    pub arg_spans2: EmptyLabelManySpans,
}
/* AST_META: AST_ID=45 | TYPE=FUNCTION | NAME=add_to_diag | COMPLEXITY=7 | LINES=9 */

pub(crate) struct EmptyLabelManySpans(pub Vec<Span>);

// The derive for `Vec<Span>` does multiple calls to `span_label`, adding commas between each
impl Subdiagnostic for EmptyLabelManySpans {
    fn add_to_diag<G: EmissionGuarantee>(self, diag: &mut Diag<'_, G>) {
        diag.span_labels(self.0, "");
    }
}
/* AST_META: AST_ID=46 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_pattern_in_fn_pointer, code = E0561)]
pub(crate) struct PatternFnPointer {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=47 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_trait_object_single_bound, code = E0226)]
pub(crate) struct TraitObjectBound {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=48 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */

#[derive(Diagnostic)]
#[diag(ast_passes_nested_impl_trait, code = E0666)]
pub(crate) struct NestedImplTrait {
    #[primary_span]
    pub span: Span,
    #[label(ast_passes_outer)]
    pub outer: Span,
    #[label(ast_passes_inner)]
    pub inner: Span,
}
/* AST_META: AST_ID=49 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_at_least_one_trait)]
pub(crate) struct AtLeastOneTrait {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=50 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=12 */

#[derive(Diagnostic)]
#[diag(ast_passes_out_of_order_params)]
pub(crate) struct OutOfOrderParams<'a> {
    #[primary_span]
    pub spans: Vec<Span>,
    #[suggestion(code = "{ordered_params}", applicability = "machine-applicable")]
    pub sugg_span: Span,
    pub param_ord: &'a ParamKindOrd,
    pub max_param: &'a ParamKindOrd,
    pub ordered_params: &'a str,
}
/* AST_META: AST_ID=51 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(ast_passes_obsolete_auto)]
#[help]
pub(crate) struct ObsoleteAuto {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=52 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */

#[derive(Diagnostic)]
#[diag(ast_passes_unsafe_negative_impl, code = E0198)]
pub(crate) struct UnsafeNegativeImpl {
    #[primary_span]
    pub span: Span,
    #[label(ast_passes_negative)]
    pub negative: Span,
    #[label(ast_passes_unsafe)]
    pub r#unsafe: Span,
}
/* AST_META: AST_ID=53 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(ast_passes_unsafe_item)]
pub(crate) struct UnsafeItem {
    #[primary_span]
    pub span: Span,
    pub kind: &'static str,
}
/* AST_META: AST_ID=54 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_missing_unsafe_on_extern)]
pub(crate) struct MissingUnsafeOnExtern {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=55 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_fieldless_union)]
pub(crate) struct FieldlessUnion {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=56 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(ast_passes_where_clause_after_type_alias)]
#[note]
pub(crate) struct WhereClauseAfterTypeAlias {
    #[primary_span]
    pub span: Span,
    #[help]
    pub help: bool,
}
/* AST_META: AST_ID=57 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(ast_passes_where_clause_before_type_alias)]
#[note]
pub(crate) struct WhereClauseBeforeTypeAlias {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub sugg: WhereClauseBeforeTypeAliasSugg,
}
/* AST_META: AST_ID=58 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=6 | LINES=21 */

#[derive(Subdiagnostic)]
pub(crate) enum WhereClauseBeforeTypeAliasSugg {
    #[suggestion(ast_passes_remove_suggestion, applicability = "machine-applicable", code = "")]
    Remove {
        #[primary_span]
        span: Span,
    },
    #[multipart_suggestion(
        ast_passes_move_suggestion,
        applicability = "machine-applicable",
        style = "verbose"
    )]
    Move {
        #[suggestion_part(code = "")]
        left: Span,
        snippet: String,
        #[suggestion_part(code = "{snippet}")]
        right: Span,
    },
}
/* AST_META: AST_ID=59 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_generic_default_trailing)]
pub(crate) struct GenericDefaultTrailing {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=60 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_nested_lifetimes, code = E0316)]
pub(crate) struct NestedLifetimes {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=61 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_const_bound_trait_object)]
pub(crate) struct ConstBoundTraitObject {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=62 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */

// FIXME(const_trait_impl): Consider making the note/reason the message of the diagnostic.
// FIXME(const_trait_impl): Provide structured suggestions (e.g., add `const` here).
#[derive(Diagnostic)]
#[diag(ast_passes_tilde_const_disallowed)]
pub(crate) struct TildeConstDisallowed {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub reason: TildeConstReason,
}
/* AST_META: AST_ID=63 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=15 | LINES=65 */

#[derive(Subdiagnostic, Copy, Clone)]
pub(crate) enum TildeConstReason {
    #[note(ast_passes_closure)]
    Closure,
    #[note(ast_passes_function)]
    Function {
        #[primary_span]
        ident: Span,
    },
    #[note(ast_passes_trait)]
    Trait {
        #[primary_span]
        span: Span,
    },
    #[note(ast_passes_trait_impl)]
    TraitImpl {
        #[primary_span]
        span: Span,
    },
    #[note(ast_passes_impl)]
    Impl {
        #[primary_span]
        span: Span,
    },
    #[note(ast_passes_trait_assoc_ty)]
    TraitAssocTy {
        #[primary_span]
        span: Span,
    },
    #[note(ast_passes_trait_impl_assoc_ty)]
    TraitImplAssocTy {
        #[primary_span]
        span: Span,
    },
    #[note(ast_passes_inherent_assoc_ty)]
    InherentAssocTy {
        #[primary_span]
        span: Span,
    },
    #[note(ast_passes_struct)]
    Struct {
        #[primary_span]
        span: Span,
    },
    #[note(ast_passes_enum)]
    Enum {
        #[primary_span]
        span: Span,
    },
    #[note(ast_passes_union)]
    Union {
        #[primary_span]
        span: Span,
    },
    #[note(ast_passes_anon_const)]
    AnonConst {
        #[primary_span]
        span: Span,
    },
    #[note(ast_passes_object)]
    TraitObject,
    #[note(ast_passes_item)]
    Item,
}
/* AST_META: AST_ID=64 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=14 */

#[derive(Diagnostic)]
#[diag(ast_passes_const_and_coroutine)]
pub(crate) struct ConstAndCoroutine {
    #[primary_span]
    pub spans: Vec<Span>,
    #[label(ast_passes_const)]
    pub const_span: Span,
    #[label(ast_passes_coroutine)]
    pub coroutine_span: Span,
    #[label]
    pub span: Span,
    pub coroutine_kind: &'static str,
}
/* AST_META: AST_ID=65 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */

#[derive(Diagnostic)]
#[diag(ast_passes_const_and_c_variadic)]
pub(crate) struct ConstAndCVariadic {
    #[primary_span]
    pub spans: Vec<Span>,
    #[label(ast_passes_const)]
    pub const_span: Span,
    #[label(ast_passes_variadic)]
    pub variadic_span: Span,
}
/* AST_META: AST_ID=66 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12 */

#[derive(Diagnostic)]
#[diag(ast_passes_coroutine_and_c_variadic)]
pub(crate) struct CoroutineAndCVariadic {
    #[primary_span]
    pub spans: Vec<Span>,
    pub coroutine_kind: &'static str,
    #[label(ast_passes_const)]
    pub coroutine_span: Span,
    #[label(ast_passes_variadic)]
    pub variadic_span: Span,
}
/* AST_META: AST_ID=67 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_pattern_in_foreign, code = E0130)]
// FIXME: deduplicate with rustc_lint (`BuiltinLintDiag::PatternsInFnsWithoutBody`)
pub(crate) struct PatternInForeign {
    #[primary_span]
    #[label]
    pub span: Span,
}
/* AST_META: AST_ID=68 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_pattern_in_bodiless, code = E0642)]
// FIXME: deduplicate with rustc_lint (`BuiltinLintDiag::PatternsInFnsWithoutBody`)
pub(crate) struct PatternInBodiless {
    #[primary_span]
    #[label]
    pub span: Span,
}
/* AST_META: AST_ID=69 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=13 */

#[derive(Diagnostic)]
#[diag(ast_passes_equality_in_where)]
#[note]
pub(crate) struct EqualityInWhere {
    #[primary_span]
    #[label]
    pub span: Span,
    #[subdiagnostic]
    pub assoc: Option<AssociatedSuggestion>,
    #[subdiagnostic]
    pub assoc2: Option<AssociatedSuggestion2>,
}
/* AST_META: AST_ID=70 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=15 */

#[derive(Subdiagnostic)]
#[suggestion(
    ast_passes_suggestion,
    code = "{param}: {path}",
    style = "verbose",
    applicability = "maybe-incorrect"
)]
pub(crate) struct AssociatedSuggestion {
    #[primary_span]
    pub span: Span,
    pub ident: Ident,
    pub param: Ident,
    pub path: String,
}
/* AST_META: AST_ID=71 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=12 */

#[derive(Subdiagnostic)]
#[multipart_suggestion(ast_passes_suggestion_path, applicability = "maybe-incorrect")]
pub(crate) struct AssociatedSuggestion2 {
    #[suggestion_part(code = "{args}")]
    pub span: Span,
    pub args: String,
    #[suggestion_part(code = "")]
    pub predicate: Span,
    pub trait_segment: Ident,
    pub potential_assoc: Ident,
}
/* AST_META: AST_ID=72 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12 */

#[derive(Diagnostic)]
#[diag(ast_passes_feature_on_non_nightly, code = E0554)]
pub(crate) struct FeatureOnNonNightly {
    #[primary_span]
    pub span: Span,
    pub channel: &'static str,
    #[subdiagnostic]
    pub stable_features: Vec<StableFeature>,
    #[suggestion(code = "", applicability = "machine-applicable")]
    pub sugg: Option<Span>,
}
/* AST_META: AST_ID=73 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */

pub(crate) struct StableFeature {
    pub name: Symbol,
    pub since: Symbol,
}
/* AST_META: AST_ID=74 | TYPE=FUNCTION | NAME=add_to_diag | COMPLEXITY=5 | LINES=8 */

impl Subdiagnostic for StableFeature {
    fn add_to_diag<G: EmissionGuarantee>(self, diag: &mut Diag<'_, G>) {
        diag.arg("name", self.name);
        diag.arg("since", self.since);
        diag.help(fluent::ast_passes_stable_since);
    }
}
/* AST_META: AST_ID=75 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(ast_passes_incompatible_features)]
#[help]
pub(crate) struct IncompatibleFeatures {
    #[primary_span]
    pub spans: Vec<Span>,
    pub f1: Symbol,
    pub f2: Symbol,
}
/* AST_META: AST_ID=76 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_negative_bound_not_supported)]
pub(crate) struct NegativeBoundUnsupported {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=77 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_constraint_on_negative_bound)]
pub(crate) struct ConstraintOnNegativeBound {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=78 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(ast_passes_negative_bound_with_parenthetical_notation)]
pub(crate) struct NegativeBoundWithParentheticalNotation {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=79 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=9 | LINES=16 */

#[derive(Diagnostic)]
#[diag(ast_passes_match_arm_with_no_body)]
pub(crate) struct MatchArmWithNoBody {
    #[primary_span]
    pub span: Span,
    // We include the braces around `todo!()` so that a comma is optional, and we don't have to have
    // any logic looking at the arm being replaced if there was a comma already or not for the
    // resulting code to be correct.
    #[suggestion(
        code = " => {{ todo!() }}",
        applicability = "has-placeholders",
        style = "verbose"
    )]
    pub suggestion: Span,
}
/* AST_META: AST_ID=80 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(ast_passes_precise_capturing_not_allowed_here)]
pub(crate) struct PreciseCapturingNotAllowedHere {
    #[primary_span]
    pub span: Span,
    pub loc: &'static str,
}
/* AST_META: AST_ID=81 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_precise_capturing_duplicated)]
pub(crate) struct DuplicatePreciseCapturing {
    #[primary_span]
    pub bound1: Span,
    #[label]
    pub bound2: Span,
}
/* AST_META: AST_ID=82 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_extern_without_abi)]
#[help]
pub(crate) struct MissingAbi {
    #[primary_span]
    #[suggestion(code = "extern \"<abi>\"", applicability = "has-placeholders")]
    pub span: Span,
}
/* AST_META: AST_ID=83 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=8 */

#[derive(LintDiagnostic)]
#[diag(ast_passes_extern_without_abi_sugg)]
pub(crate) struct MissingAbiSugg {
    #[suggestion(code = "extern {default_abi}", applicability = "machine-applicable")]
    pub span: Span,
    pub default_abi: ExternAbi,
}
/* AST_META: AST_ID=84 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=15 */

#[derive(Diagnostic)]
#[diag(ast_passes_abi_custom_safe_foreign_function)]
pub(crate) struct AbiCustomSafeForeignFunction {
    #[primary_span]
    pub span: Span,

    #[suggestion(
        ast_passes_suggestion,
        applicability = "maybe-incorrect",
        code = "",
        style = "verbose"
    )]
    pub safe_span: Span,
}
/* AST_META: AST_ID=85 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=6 | LINES=16 */

#[derive(Diagnostic)]
#[diag(ast_passes_abi_custom_safe_function)]
pub(crate) struct AbiCustomSafeFunction {
    #[primary_span]
    pub span: Span,
    pub abi: ExternAbi,

    #[suggestion(
        ast_passes_suggestion,
        applicability = "maybe-incorrect",
        code = "unsafe ",
        style = "verbose"
    )]
    pub unsafe_span: Span,
}
/* AST_META: AST_ID=86 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=17 */

#[derive(Diagnostic)]
#[diag(ast_passes_abi_cannot_be_coroutine)]
pub(crate) struct AbiCannotBeCoroutine {
    #[primary_span]
    pub span: Span,
    pub abi: ExternAbi,

    #[suggestion(
        ast_passes_suggestion,
        applicability = "maybe-incorrect",
        code = "",
        style = "verbose"
    )]
    pub coroutine_kind_span: Span,
    pub coroutine_kind_str: &'static str,
}
/* AST_META: AST_ID=87 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=19 */

#[derive(Diagnostic)]
#[diag(ast_passes_abi_must_not_have_parameters_or_return_type)]
#[note]
pub(crate) struct AbiMustNotHaveParametersOrReturnType {
    #[primary_span]
    pub spans: Vec<Span>,
    pub abi: ExternAbi,

    #[suggestion(
        ast_passes_suggestion,
        applicability = "maybe-incorrect",
        code = "{padding}fn {symbol}()",
        style = "verbose"
    )]
    pub suggestion_span: Span,
    pub symbol: Symbol,
    pub padding: &'static str,
}
/* AST_META: AST_ID=88 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(ast_passes_abi_must_not_have_return_type)]
#[note]
pub(crate) struct AbiMustNotHaveReturnType {
    #[primary_span]
    #[help]
    pub span: Span,
    pub abi: ExternAbi,
}
/* AST_META: AST_ID=89 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(ast_passes_abi_x86_interrupt)]
#[note]
pub(crate) struct AbiX86Interrupt {
    #[primary_span]
    pub spans: Vec<Span>,
    pub param_count: usize,
}