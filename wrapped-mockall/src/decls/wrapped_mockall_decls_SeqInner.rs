use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Default)]
struct SeqInner {
    /// Should match the `seq` field of the next [`SeqHandle`] that has not been
    /// fully satisfied.
    satisfaction_level: AtomicUsize,
}
