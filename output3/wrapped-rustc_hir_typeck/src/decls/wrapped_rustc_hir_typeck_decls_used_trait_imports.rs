use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn used_trait_imports(tcx: TyCtxt<'_>, def_id: LocalDefId) -> &UnordSet<LocalDefId> {
    &tcx.typeck(def_id).used_trait_imports
}
