// SRC: ../rust/compiler/rustc_lint/src/redundant_semicolon.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::{Block, StmtKind};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::{declare_lint, declare_lint_pass};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use crate::rustc_complete::Span;

use crate::lints::{RedundantSemicolonsDiag, RedundantSemicolonsSuggestion};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::{EarlyContext, EarlyLintPass, LintContext};
/* AST_META: AST_ID=5 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=21 */

declare_lint! {
    /// The `redundant_semicolons` lint detects unnecessary trailing
    /// semicolons.
    ///
    /// ### Example
    ///
    /// ```rust
    /// let _ = 123;;
    /// ```
    ///
    /// {{produces}}
    ///
    /// ### Explanation
    ///
    /// Extra semicolons are not needed, and may be removed to avoid confusion
    /// and visual clutter.
    pub REDUNDANT_SEMICOLONS,
    Warn,
    "detects unnecessary trailing semicolons"
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=check_block | COMPLEXITY=13 | LINES=16 */

declare_lint_pass!(RedundantSemicolons => [REDUNDANT_SEMICOLONS]);

impl EarlyLintPass for RedundantSemicolons {
    fn check_block(&mut self, cx: &EarlyContext<'_>, block: &Block) {
        let mut seq = None;
        for stmt in block.stmts.iter() {
            match (&stmt.kind, &mut seq) {
                (StmtKind::Empty, None) => seq = Some((stmt.span, false)),
                (StmtKind::Empty, Some(seq)) => *seq = (seq.0.to(stmt.span), true),
                (_, seq) => maybe_lint_redundant_semis(cx, seq),
            }
        }
        maybe_lint_redundant_semis(cx, &mut seq);
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=maybe_lint_redundant_semis | COMPLEXITY=15 | LINES=21 */

fn maybe_lint_redundant_semis(cx: &EarlyContext<'_>, seq: &mut Option<(Span, bool)>) {
    if let Some((span, multiple)) = seq.take() {
        if span == crate::rustc_span::DUMMY_SP {
            return;
        }

        // Ignore redundant semicolons inside macro expansion.(issue #142143)
        let suggestion = if span.from_expansion() {
            None
        } else {
            Some(RedundantSemicolonsSuggestion { multiple_semicolons: multiple, span })
        };

        cx.emit_span_lint(
            REDUNDANT_SEMICOLONS,
            span,
            RedundantSemicolonsDiag { multiple, suggestion },
        );
    }
}