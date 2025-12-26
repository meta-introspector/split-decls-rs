use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Copy, Debug)]
enum RelaxedBoundForbiddenReason {
    TraitObjectTy,
    SuperTrait,
    TraitAlias,
    AssocTyBounds,
    LateBoundVarsInScope,
}
