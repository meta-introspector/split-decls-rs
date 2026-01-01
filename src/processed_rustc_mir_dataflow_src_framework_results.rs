// SRC: ../rust/compiler/rustc_mir_dataflow/src/framework/results.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
// Dataflow analysis results.

use crate::rustc_index::IndexVec;
use crate::rustc_complete::mir::{BasicBlock, Body};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

use super::{Analysis, ResultsCursor};
/* AST_META: AST_ID=3 | TYPE=STRUCT | NAME=AnalysisAndResults | COMPLEXITY=3 | LINES=14 */

/// The results of a dataflow analysis that has converged to fixpoint. It only holds the domain
/// values at the entry of each basic block. Domain values in other parts of the block are
/// recomputed on the fly by visitors (i.e. `ResultsCursor`, or `ResultsVisitor` impls).
pub type Results<D> = IndexVec<BasicBlock, D>;

/// Utility type used in a few places where it's convenient to bundle an analysis with its results.
pub struct AnalysisAndResults<'tcx, A>
where
    A: Analysis<'tcx>,
{
    pub analysis: A,
    pub results: Results<A::Domain>,
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=into_results_cursor | COMPLEXITY=3 | LINES=10 */

impl<'tcx, A> AnalysisAndResults<'tcx, A>
where
    A: Analysis<'tcx>,
{
    /// Creates a `ResultsCursor` that takes ownership of `self`.
    pub fn into_results_cursor<'mir>(self, body: &'mir Body<'tcx>) -> ResultsCursor<'mir, 'tcx, A> {
        ResultsCursor::new_owning(body, self.analysis, self.results)
    }
}