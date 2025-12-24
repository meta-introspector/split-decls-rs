use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Defines system native-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// On this platform, this is an alias for [`BigEndian`].
///
/// [`BigEndian`]: enum.BigEndian.html
#[cfg(target_endian = "big")]
pub type NativeEndian = BigEndian;
