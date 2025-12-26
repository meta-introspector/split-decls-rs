use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Runs `A` and `B` side by side and only yields items present in `B`
struct DifferenceIter<L, R, F> {
    left: Fuse<L>,
    right: R,
    compare: F,
}
