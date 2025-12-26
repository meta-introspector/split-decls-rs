use serde::{Deserialize, Serialize};
use std::collections::HashMap;
static ERROR_FIELDS: Lazy<Fields> = Lazy::new(|| Fields::new(&ERROR_CS));
