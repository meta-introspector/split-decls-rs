// SRC: ../rust/compiler/rustc_borrowck/src/prefixes.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=6 | LINES=7 */
// From the NLL RFC:
// "Shallow prefixes are found by stripping away fields, but stop at
// any dereference. So: writing a path like `a` is illegal if `a.b`
// is borrowed. But: writing `a` is legal if `*a` is borrowed,
// whether or not `a` is a shared or mutable reference. [...] "

use crate::rustc_complete::mir::{PlaceRef, ProjectionElem};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=is_prefix_of | COMPLEXITY=2 | LINES=6 */

use super::MirBorrowckCtxt;

pub(crate) trait IsPrefixOf<'tcx> {
    fn is_prefix_of(&self, other: PlaceRef<'tcx>) -> bool;
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=is_prefix_of | COMPLEXITY=5 | LINES=8 */

impl<'tcx> IsPrefixOf<'tcx> for PlaceRef<'tcx> {
    fn is_prefix_of(&self, other: PlaceRef<'tcx>) -> bool {
        self.local == other.local
            && self.projection.len() <= other.projection.len()
            && self.projection == &other.projection[..self.projection.len()]
    }
}
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */

pub(super) struct Prefixes<'tcx> {
    kind: PrefixSet,
    next: Option<PlaceRef<'tcx>>,
}
/* AST_META: AST_ID=5 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(super) enum PrefixSet {
    /// Doesn't stop until it returns the base case (a Local or
    /// Static prefix).
    All,
    /// Stops at any dereference.
    Shallow,
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=9 */

impl<'tcx> MirBorrowckCtxt<'_, '_, 'tcx> {
    /// Returns an iterator over the prefixes of `place`
    /// (inclusive) from longest to smallest, potentially
    /// terminating the iteration early based on `kind`.
    pub(super) fn prefixes(&self, place_ref: PlaceRef<'tcx>, kind: PrefixSet) -> Prefixes<'tcx> {
        Prefixes { next: Some(place_ref), kind }
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=next | COMPLEXITY=37 | LINES=61 */

impl<'tcx> Iterator for Prefixes<'tcx> {
    type Item = PlaceRef<'tcx>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut cursor = self.next?;

        // Post-processing `place`: Enqueue any remaining
        // work. Also, `place` may not be a prefix itself, but
        // may hold one further down (e.g., we never return
        // downcasts here, but may return a base of a downcast).

        loop {
            match cursor.last_projection() {
                None => {
                    self.next = None;
                    return Some(cursor);
                }
                Some((cursor_base, elem)) => {
                    match elem {
                        ProjectionElem::Field(_ /*field*/, _ /*ty*/) => {
                            // FIXME: add union handling
                            self.next = Some(cursor_base);
                            return Some(cursor);
                        }
                        ProjectionElem::UnwrapUnsafeBinder(_) => {
                            self.next = Some(cursor_base);
                            return Some(cursor);
                        }
                        ProjectionElem::Downcast(..)
                        | ProjectionElem::Subslice { .. }
                        | ProjectionElem::OpaqueCast { .. }
                        | ProjectionElem::ConstantIndex { .. }
                        | ProjectionElem::Index(_) => {
                            cursor = cursor_base;
                        }
                        ProjectionElem::Subtype(..) => {
                            panic!("Subtype projection is not allowed before borrow check")
                        }
                        ProjectionElem::Deref => {
                            match self.kind {
                                PrefixSet::Shallow => {
                                    // Shallow prefixes are found by stripping away
                                    // fields, but stop at *any* dereference.
                                    // So we can just stop the traversal now.
                                    self.next = None;
                                    return Some(cursor);
                                }
                                PrefixSet::All => {
                                    // All prefixes: just blindly enqueue the base
                                    // of the projection.
                                    self.next = Some(cursor_base);
                                    return Some(cursor);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}