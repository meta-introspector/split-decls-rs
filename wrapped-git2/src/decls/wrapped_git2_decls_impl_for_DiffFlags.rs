use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl DiffFlags {
    is_bit_set!(is_binary, DiffFlags::BINARY);
    is_bit_set!(is_not_binary, DiffFlags::NOT_BINARY);
    is_bit_set!(has_valid_id, DiffFlags::VALID_ID);
    is_bit_set!(exists, DiffFlags::EXISTS);
}
