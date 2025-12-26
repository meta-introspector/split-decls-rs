use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct LazyDefPathStr<'tcx> {
    def_id: DefId,
    tcx: TyCtxt<'tcx>,
}
