use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'tcx> DefIdVisitor<'tcx> for SearchInterfaceForPrivateItemsVisitor<'tcx> {
    type Result = ControlFlow<()>;
    fn skip_assoc_tys(&self) -> bool {
        self.skip_assoc_tys
    }
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
    fn visit_def_id(
        &mut self,
        def_id: DefId,
        kind: &str,
        descr: &dyn fmt::Display,
    ) -> Self::Result {
        if self.check_def_id(def_id, kind, descr) {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }
}
