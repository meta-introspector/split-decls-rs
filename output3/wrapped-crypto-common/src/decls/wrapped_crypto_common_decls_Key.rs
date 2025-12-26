use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Key used by [`KeySizeUser`] implementors.
pub type Key<B> = Array<u8, <B as KeySizeUser>::KeySize>;
