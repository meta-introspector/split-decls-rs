use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct ReachEverythingInTheInterfaceVisitor<'a, 'tcx> {
    effective_vis: EffectiveVisibility,
    item_def_id: LocalDefId,
    ev: &'a mut EmbargoVisitor<'tcx>,
    level: Level,
}
