use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct EnvChange<'snap> {
    changed_vars: Vec<&'snap str>,
    prev_working_dir: Option<PathBuf>,
    snap: &'snap EnvSnapshot,
    _guard: std::sync::MutexGuard<'snap, ()>,
}
