use serde::{Deserialize, Serialize};
use std::collections::HashMap;
type DeferredClosureRequirements<'tcx> = Vec<(LocalDefId, ty::GenericArgsRef<'tcx>, Locations)>;
