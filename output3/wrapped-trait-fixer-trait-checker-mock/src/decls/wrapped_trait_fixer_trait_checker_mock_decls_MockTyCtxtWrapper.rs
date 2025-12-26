use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub struct MockTyCtxtWrapper<'tcx>(pub TyCtxt<'tcx>);
