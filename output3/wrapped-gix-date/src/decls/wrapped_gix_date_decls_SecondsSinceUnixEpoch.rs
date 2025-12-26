use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The number of seconds since unix epoch.
///
/// Note that negative dates represent times before the unix epoch.
///
/// ### Deviation
///
/// `git` only supports dates *from* the UNIX epoch, whereas we chose to be more flexible at the expense of stopping time
/// a few million years before the heat-death of the universe.
pub type SecondsSinceUnixEpoch = i64;
