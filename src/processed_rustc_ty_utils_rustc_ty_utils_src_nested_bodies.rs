// SRC: ../rust/compiler/rustc_ty_utils/src/nested_bodies.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use rustc_hir as hir;
use crate::rustc_complete::def_id::{DefId, LocalDefId};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use crate::rustc_complete::intravisit::Visitor;
use crate::rustc_complete::query::Providers;
use crate::rustc_complete::ty::{self, TyCtxt};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=nested_bodies_within | COMPLEXITY=3 | LINES=8 */

fn nested_bodies_within<'tcx>(tcx: TyCtxt<'tcx>, item: LocalDefId) -> &'tcx ty::List<LocalDefId> {
    let body = tcx.hir_body_owned_by(item);
    let mut collector =
        NestedBodiesVisitor { tcx, root_def_id: item.to_def_id(), nested_bodies: vec![] };
    collector.visit_body(body);
    tcx.mk_local_def_ids(&collector.nested_bodies)
}
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=NestedBodiesVisitor | COMPLEXITY=2 | LINES=6 */

struct NestedBodiesVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    root_def_id: DefId,
    nested_bodies: Vec<LocalDefId>,
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=visit_nested_body | COMPLEXITY=9 | LINES=13 */

impl<'tcx> Visitor<'tcx> for NestedBodiesVisitor<'tcx> {
    fn visit_nested_body(&mut self, id: hir::BodyId) {
        let body_def_id = self.tcx.hir_body_owner_def_id(id);
        if self.tcx.typeck_root_def_id(body_def_id.to_def_id()) == self.root_def_id {
            // We visit nested bodies before adding the current body. This
            // means that nested bodies are always stored before their parent.
            let body = self.tcx.hir_body(id);
            self.visit_body(body);
            self.nested_bodies.push(body_def_id);
        }
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=4 */

pub(super) fn provide(providers: &mut Providers) {
    *providers = Providers { nested_bodies_within, ..*providers };
}