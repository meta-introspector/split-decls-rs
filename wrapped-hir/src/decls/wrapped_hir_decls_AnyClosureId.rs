use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum AnyClosureId {
    ClosureId(InternedClosureId),
    CoroutineClosureId(InternedCoroutineId),
}
