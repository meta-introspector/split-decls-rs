use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Default for CfgOptions {
    fn default() -> Self {
        Self {
            enabled: FxHashSet::from_iter([CfgAtom::Flag(sym::true_)]),
        }
    }
}
