use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub(crate) fn coder_get_dict_size(len: usize) -> usize {
    if len < DIST_STATES + MATCH_LEN_MIN { len - MATCH_LEN_MIN } else { DIST_STATES - 1 }
}
