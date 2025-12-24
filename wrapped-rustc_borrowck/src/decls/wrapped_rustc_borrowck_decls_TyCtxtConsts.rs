use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Associate some local constants with the `'tcx` lifetime
struct TyCtxtConsts<'tcx>(PhantomData<&'tcx ()>);
