use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'tcx> LangItems<'tcx, TyCtxt<'tcx>, DefId> for MockTyCtxt<'tcx> {
    fn get_clone_trait_def_id(&self) -> Option<DefId> {
        Some(DefId)
    }
    fn get_debug_trait_def_id(&self) -> Option<DefId> {
        Some(DefId)
    }
}
