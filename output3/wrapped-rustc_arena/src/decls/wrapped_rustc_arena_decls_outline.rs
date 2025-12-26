use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// This calls the passed function while ensuring it won't be inlined into the caller.
#[inline(never)]
#[cold]
fn outline<F: FnOnce() -> R, R>(f: F) -> R {
    f()
}
