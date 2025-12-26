use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Closure<'db> {
    id: AnyClosureId,
    subst: GenericArgs<'db>,
}
