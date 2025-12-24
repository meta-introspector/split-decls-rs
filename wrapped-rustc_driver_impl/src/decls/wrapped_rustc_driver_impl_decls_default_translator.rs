use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn default_translator() -> Translator {
    Translator::with_fallback_bundle(DEFAULT_LOCALE_RESOURCES.to_vec(), false)
}
