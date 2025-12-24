use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The blocking executor.
struct Executor {
    /// Inner state of the executor.
    inner: Mutex<Inner>,
    /// Used to put idle threads to sleep and wake them up when new work comes in.
    cvar: Condvar,
}
