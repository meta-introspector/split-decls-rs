use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The type for a closure that gets invoked when a thread exits. The
/// closure is passed the index of the thread on which it is invoked.
/// Note that this same closure may be invoked multiple times in parallel.
type ExitHandler = dyn Fn(usize) + Send + Sync;
