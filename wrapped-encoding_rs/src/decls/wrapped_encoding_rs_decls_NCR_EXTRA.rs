use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// This has to be the max length of an NCR instead of max
/// minus one, because we can't rely on getting the minus
/// one from the space reserved for the current unmappable,
/// because the ISO-2022-JP encoder can fill up that space
/// with a state transition escape.
const NCR_EXTRA: usize = 10;
