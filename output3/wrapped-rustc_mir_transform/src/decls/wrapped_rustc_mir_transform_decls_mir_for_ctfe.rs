use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Compute the MIR that is used during CTFE (and thus has no optimizations run on it)
fn mir_for_ctfe(tcx: TyCtxt<'_>, def_id: LocalDefId) -> &Body<'_> {
    debug_assert!(
        !tcx.is_trivial_const(def_id),
        "Tried to get mir_for_ctfe of a trivial const"
    );
    tcx.arena.alloc(inner_mir_for_ctfe(tcx, def_id))
}
