// SRC: ../rust/compiler/rustc_borrowck/src/polonius/legacy/accesses.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::mir::visit::{MutatingUseContext, PlaceContext, Visitor};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::mir::{Body, Local, Location, Place};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::ty::TyCtxt;
use crate::rustc_mir_dataflow::move_paths::{LookupResult, MoveData};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use tracing::debug;

use super::{LocationIndex, PoloniusFacts, PoloniusLocationTable};
/* AST_META: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::def_use::{self, DefUse};
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=12 | LINES=22 */
use crate::universal_regions::UniversalRegions;

/// Emit polonius facts for variable defs, uses, drops, and path accesses.
pub(crate) fn emit_access_facts<'tcx>(
    tcx: TyCtxt<'tcx>,
    facts: &mut PoloniusFacts,
    body: &Body<'tcx>,
    location_table: &PoloniusLocationTable,
    move_data: &MoveData<'tcx>,
    universal_regions: &UniversalRegions<'tcx>,
) {
    let mut extractor = AccessFactsExtractor { facts, move_data, location_table };
    extractor.visit_body(body);

    for (local, local_decl) in body.local_decls.iter_enumerated() {
        debug!("add use_of_var_derefs_origin facts - local={:?}, type={:?}", local, local_decl.ty);
        tcx.for_each_free_region(&local_decl.ty, |region| {
            let region_vid = universal_regions.to_region_vid(region);
            facts.use_of_var_derefs_origin.push((local, region_vid.into()));
        });
    }
}
/* AST_META: AST_ID=7 | TYPE=STRUCT | NAME=AccessFactsExtractor | COMPLEXITY=2 | LINES=7 */

/// MIR visitor extracting point-wise facts about accesses.
struct AccessFactsExtractor<'a, 'tcx> {
    facts: &'a mut PoloniusFacts,
    move_data: &'a MoveData<'tcx>,
    location_table: &'a PoloniusLocationTable,
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=location_to_index | COMPLEXITY=3 | LINES=6 */

impl<'tcx> AccessFactsExtractor<'_, 'tcx> {
    fn location_to_index(&self, location: Location) -> LocationIndex {
        self.location_table.mid_index(location)
    }
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=visit_local | COMPLEXITY=29 | LINES=41 */

impl<'a, 'tcx> Visitor<'tcx> for AccessFactsExtractor<'a, 'tcx> {
    fn visit_local(&mut self, local: Local, context: PlaceContext, location: Location) {
        match def_use::categorize(context) {
            Some(DefUse::Def) => {
                debug!("AccessFactsExtractor - emit def");
                self.facts.var_defined_at.push((local, self.location_to_index(location)));
            }
            Some(DefUse::Use) => {
                debug!("AccessFactsExtractor - emit use");
                self.facts.var_used_at.push((local, self.location_to_index(location)));
            }
            Some(DefUse::Drop) => {
                debug!("AccessFactsExtractor - emit drop");
                self.facts.var_dropped_at.push((local, self.location_to_index(location)));
            }
            _ => (),
        }
    }

    fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, location: Location) {
        self.super_place(place, context, location);

        match context {
            PlaceContext::NonMutatingUse(_)
            | PlaceContext::MutatingUse(MutatingUseContext::Borrow) => {
                let path = match self.move_data.rev_lookup.find(place.as_ref()) {
                    LookupResult::Exact(path) | LookupResult::Parent(Some(path)) => path,
                    _ => {
                        // There's no path access to emit.
                        return;
                    }
                };
                debug!("AccessFactsExtractor - emit path access ({path:?}, {location:?})");
                self.facts.path_accessed_at_base.push((path, self.location_to_index(location)));
            }

            _ => {}
        }
    }
}