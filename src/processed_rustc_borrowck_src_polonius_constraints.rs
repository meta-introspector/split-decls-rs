// SRC: ../rust/compiler/rustc_borrowck/src/polonius/constraints.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=6 | LINES=26 */
use crate::rustc_complete::ty::RegionVid;
use crate::rustc_mir_dataflow::points::PointIndex;

/// A localized outlives constraint reifies the CFG location where the outlives constraint holds,
/// within the origins themselves as if they were different from point to point: from `a: b`
/// outlives constraints to `a@p: b@p`, where `p` is the point in the CFG.
///
/// This models two sources of constraints:
/// - constraints that traverse the subsets between regions at a given point, `a@p: b@p`. These
///   depend on typeck constraints generated via assignments, calls, etc.
/// - constraints that traverse the CFG via the same region, `a@p: a@q`, where `p` is a predecessor
///   of `q`. These depend on the liveness of the regions at these points, as well as their
///   variance.
///
/// The `source` origin at `from` flows into the `target` origin at `to`.
///
/// This dual of NLL's [crate::constraints::OutlivesConstraint] therefore encodes the
/// position-dependent outlives constraints used by Polonius, to model the flow-sensitive loan
/// propagation via reachability within a graph of localized constraints.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub(crate) struct LocalizedOutlivesConstraint {
    pub source: RegionVid,
    pub from: PointIndex,
    pub target: RegionVid,
    pub to: PointIndex,
}
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

/// A container of [LocalizedOutlivesConstraint]s that can be turned into a traversable
/// `rustc_data_structures` graph.
#[derive(Clone, Default, Debug)]
pub(crate) struct LocalizedOutlivesConstraintSet {
    pub outlives: Vec<LocalizedOutlivesConstraint>,
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=10 */

impl LocalizedOutlivesConstraintSet {
    pub(crate) fn push(&mut self, constraint: LocalizedOutlivesConstraint) {
        if constraint.source == constraint.target && constraint.from == constraint.to {
            // 'a@p: 'a@p is pretty uninteresting
            return;
        }
        self.outlives.push(constraint);
    }
}