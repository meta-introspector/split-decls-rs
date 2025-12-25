use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(target_pointer_width = "64")]
rustc_data_structures::static_assert_size!(PResult<'_, bool>, 24);
