// SRC: ../rust/compiler/rustc_pattern_analysis/src/errors.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::{Diag, EmissionGuarantee, Subdiagnostic};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use rustc_macros::{Diagnostic, LintDiagnostic, Subdiagnostic};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
use crate::rustc_complete::ty::Ty;
use crate::rustc_complete::Span;

use crate::rustc::{RustcPatCtxt, WitnessPat};
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=Uncovered | COMPLEXITY=2 | LINES=12 */

#[derive(Subdiagnostic)]
#[label(pattern_analysis_uncovered)]
pub struct Uncovered {
    #[primary_span]
    span: Span,
    count: usize,
    witness_1: String, // a printed pattern
    witness_2: String, // a printed pattern
    witness_3: String, // a printed pattern
    remainder: usize,
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=new | COMPLEXITY=7 | LINES=22 */

impl Uncovered {
    pub fn new<'p, 'tcx>(
        span: Span,
        cx: &RustcPatCtxt<'p, 'tcx>,
        witnesses: Vec<WitnessPat<'p, 'tcx>>,
    ) -> Self
    where
        'tcx: 'p,
    {
        let witness_1 = cx.print_witness_pat(witnesses.get(0).unwrap());
        Self {
            span,
            count: witnesses.len(),
            // Substitute dummy values if witnesses is smaller than 3. These will never be read.
            witness_2: witnesses.get(1).map(|w| cx.print_witness_pat(w)).unwrap_or_default(),
            witness_3: witnesses.get(2).map(|w| cx.print_witness_pat(w)).unwrap_or_default(),
            witness_1,
            remainder: witnesses.len().saturating_sub(3),
        }
    }
}
/* AST_META: AST_ID=6 | TYPE=STRUCT | NAME=OverlappingRangeEndpoints | COMPLEXITY=2 | LINES=10 */

#[derive(LintDiagnostic)]
#[diag(pattern_analysis_overlapping_range_endpoints)]
#[note]
pub struct OverlappingRangeEndpoints {
    #[label]
    pub range: Span,
    #[subdiagnostic]
    pub overlap: Vec<Overlap>,
}
/* AST_META: AST_ID=7 | TYPE=STRUCT | NAME=Overlap | COMPLEXITY=2 | LINES=5 */

pub struct Overlap {
    pub span: Span,
    pub range: String, // a printed pattern
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=add_to_diag | COMPLEXITY=7 | LINES=11 */

impl Subdiagnostic for Overlap {
    fn add_to_diag<G: EmissionGuarantee>(self, diag: &mut Diag<'_, G>) {
        let Overlap { span, range } = self;

        // FIXME(mejrs) unfortunately `#[derive(LintDiagnostic)]`
        // does not support `#[subdiagnostic(eager)]`...
        let message = format!("this range overlaps on `{range}`...");
        diag.span_label(span, message);
    }
}
/* AST_META: AST_ID=9 | TYPE=STRUCT | NAME=ExclusiveRangeMissingMax | COMPLEXITY=6 | LINES=12 */

#[derive(LintDiagnostic)]
#[diag(pattern_analysis_excluside_range_missing_max)]
pub struct ExclusiveRangeMissingMax {
    #[label]
    #[suggestion(code = "{suggestion}", applicability = "maybe-incorrect")]
    /// This is an exclusive range that looks like `lo..max` (i.e. doesn't match `max`).
    pub first_range: Span,
    /// Suggest `lo..=max` instead.
    pub suggestion: String,
    pub max: String, // a printed pattern
}
/* AST_META: AST_ID=10 | TYPE=STRUCT | NAME=ExclusiveRangeMissingGap | COMPLEXITY=7 | LINES=15 */

#[derive(LintDiagnostic)]
#[diag(pattern_analysis_excluside_range_missing_gap)]
pub struct ExclusiveRangeMissingGap {
    #[label]
    #[suggestion(code = "{suggestion}", applicability = "maybe-incorrect")]
    /// This is an exclusive range that looks like `lo..gap` (i.e. doesn't match `gap`).
    pub first_range: Span,
    pub gap: String, // a printed pattern
    /// Suggest `lo..=gap` instead.
    pub suggestion: String,
    #[subdiagnostic]
    /// All these ranges skipped over `gap` which we think is probably a mistake.
    pub gap_with: Vec<GappedRange>,
}
/* AST_META: AST_ID=11 | TYPE=STRUCT | NAME=GappedRange | COMPLEXITY=2 | LINES=6 */

pub struct GappedRange {
    pub span: Span,
    pub gap: String,         // a printed pattern
    pub first_range: String, // a printed pattern
}
/* AST_META: AST_ID=12 | TYPE=FUNCTION | NAME=add_to_diag | COMPLEXITY=9 | LINES=14 */

impl Subdiagnostic for GappedRange {
    fn add_to_diag<G: EmissionGuarantee>(self, diag: &mut Diag<'_, G>) {
        let GappedRange { span, gap, first_range } = self;

        // FIXME(mejrs) unfortunately `#[derive(LintDiagnostic)]`
        // does not support `#[subdiagnostic(eager)]`...
        let message = format!(
            "this could appear to continue range `{first_range}`, but `{gap}` isn't matched by \
            either of them"
        );
        diag.span_label(span, message);
    }
}
/* AST_META: AST_ID=13 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(LintDiagnostic)]
#[diag(pattern_analysis_non_exhaustive_omitted_pattern)]
#[help]
#[note]
pub(crate) struct NonExhaustiveOmittedPattern<'tcx> {
    pub scrut_ty: Ty<'tcx>,
    #[subdiagnostic]
    pub uncovered: Uncovered,
}
/* AST_META: AST_ID=14 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=12 */

#[derive(LintDiagnostic)]
#[diag(pattern_analysis_non_exhaustive_omitted_pattern_lint_on_arm)]
#[help]
pub(crate) struct NonExhaustiveOmittedPatternLintOnArm {
    #[label]
    pub lint_span: Span,
    #[suggestion(code = "#[{lint_level}({lint_name})]\n", applicability = "maybe-incorrect")]
    pub suggest_lint_on_match: Option<Span>,
    pub lint_level: &'static str,
    pub lint_name: &'static str,
}
/* AST_META: AST_ID=15 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12 */

#[derive(Diagnostic)]
#[diag(pattern_analysis_mixed_deref_pattern_constructors)]
pub(crate) struct MixedDerefPatternConstructors<'tcx> {
    #[primary_span]
    pub spans: Vec<Span>,
    pub smart_pointer_ty: Ty<'tcx>,
    #[label(pattern_analysis_deref_pattern_label)]
    pub deref_pattern_label: Span,
    #[label(pattern_analysis_normal_constructor_label)]
    pub normal_constructor_label: Span,
}