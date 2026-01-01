// SRC: ../rust/compiler/rustc_borrowck/src/diagnostics/find_all_local_uses.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use std::collections::BTreeSet;

use crate::rustc_complete::mir::visit::{PlaceContext, Visitor};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::mir::{Body, Local, Location};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=9 */

/// Find all uses of (including assignments to) a [`Local`].
///
/// Uses `BTreeSet` so output is deterministic.
pub(super) fn find(body: &Body<'_>, local: Local) -> BTreeSet<Location> {
    let mut visitor = AllLocalUsesVisitor { for_local: local, uses: BTreeSet::default() };
    visitor.visit_body(body);
    visitor.uses
}
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=AllLocalUsesVisitor | COMPLEXITY=2 | LINES=5 */

struct AllLocalUsesVisitor {
    for_local: Local,
    uses: BTreeSet<Location>,
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=visit_local | COMPLEXITY=8 | LINES=8 */

impl<'tcx> Visitor<'tcx> for AllLocalUsesVisitor {
    fn visit_local(&mut self, local: Local, _context: PlaceContext, location: Location) {
        if local == self.for_local {
            self.uses.insert(location);
        }
    }
}