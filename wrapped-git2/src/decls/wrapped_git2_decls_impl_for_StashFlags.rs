use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl StashFlags {
    is_bit_set!(is_default, StashFlags::DEFAULT);
    is_bit_set!(is_keep_index, StashFlags::KEEP_INDEX);
    is_bit_set!(is_include_untracked, StashFlags::INCLUDE_UNTRACKED);
    is_bit_set!(is_include_ignored, StashFlags::INCLUDE_IGNORED);
}
