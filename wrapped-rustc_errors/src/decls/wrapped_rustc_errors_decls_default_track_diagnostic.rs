use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn default_track_diagnostic<R>(diag: DiagInner, f: &mut dyn FnMut(DiagInner) -> R) -> R {
    (*f)(diag)
}
