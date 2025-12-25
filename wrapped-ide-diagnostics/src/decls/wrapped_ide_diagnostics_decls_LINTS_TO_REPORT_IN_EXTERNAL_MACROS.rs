use serde::{Deserialize, Serialize};
use std::collections::HashMap;
static LINTS_TO_REPORT_IN_EXTERNAL_MACROS: LazyLock<FxHashSet<&str>> =
    LazyLock::new(|| FxHashSet::from_iter([]));
