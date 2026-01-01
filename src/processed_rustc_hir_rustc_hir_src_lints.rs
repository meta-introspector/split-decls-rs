// SRC: ../rust/compiler/rustc_hir/src/lints.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */
use crate::rustc_data_structures::fingerprint::Fingerprint;
use rustc_macros::HashStable_Generic;
use crate::rustc_complete::Span;

use crate::{AttrPath, HirId, Target};
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=DelayedLints | COMPLEXITY=2 | LINES=7 */

#[derive(Debug)]
pub struct DelayedLints {
    pub lints: Box<[DelayedLint]>,
    // Only present when the crate hash is needed.
    pub opt_hash: Option<Fingerprint>,
}
/* AST_META: AST_ID=3 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=3 | LINES=11 */

/// During ast lowering, no lints can be emitted.
/// That is because lints attach to nodes either in the AST, or on the built HIR.
/// When attached to AST nodes, they're emitted just before building HIR,
/// and then there's a gap where no lints can be emitted until HIR is done.
/// The variants in this enum represent lints that are temporarily stashed during
/// AST lowering to be emitted once HIR is built.
#[derive(Clone, Debug, HashStable_Generic)]
pub enum DelayedLint {
    AttributeParsing(AttributeLint<HirId>),
}
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=AttributeLint | COMPLEXITY=2 | LINES=7 */

#[derive(Clone, Debug, HashStable_Generic)]
pub struct AttributeLint<Id> {
    pub id: Id,
    pub span: Span,
    pub kind: AttributeLintKind,
}
/* AST_META: AST_ID=5 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=7 | LINES=9 */

#[derive(Clone, Debug, HashStable_Generic)]
pub enum AttributeLintKind {
    UnusedDuplicate { this: Span, other: Span, warning: bool },
    IllFormedAttributeInput { suggestions: Vec<String> },
    EmptyAttribute { first_span: Span },
    InvalidTarget { name: AttrPath, target: Target, applied: Vec<String>, only: &'static str },
    InvalidStyle { name: AttrPath, is_used_as_inner: bool, target: Target, target_span: Span },
}