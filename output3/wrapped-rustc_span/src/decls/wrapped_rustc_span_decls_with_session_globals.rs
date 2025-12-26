use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[inline]
pub fn with_session_globals<R, F>(f: F) -> R
where
    F: FnOnce(&SessionGlobals) -> R,
{
    SESSION_GLOBALS.with(f)
}
