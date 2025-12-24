use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[doc(hidden)]
#[deprecated(note = "use `pest::unicode::unicode_property_names` instead")]
pub static UNICODE_PROPERTY_NAMES: LazyLock<Vec<&str>> = LazyLock::new(|| {
    unicode_property_names().collect::<Vec<_>>()
});
