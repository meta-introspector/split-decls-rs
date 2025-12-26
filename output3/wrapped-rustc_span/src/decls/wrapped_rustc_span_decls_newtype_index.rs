use serde::{Deserialize, Serialize};
use std::collections::HashMap;
rustc_index::newtype_index! {
    #[orderable] #[debug_format = "AttrId({})"] pub struct AttrId {}
}
