use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl StashApplyFlags {
    is_bit_set!(is_default, StashApplyFlags::DEFAULT);
    is_bit_set!(is_reinstate_index, StashApplyFlags::REINSTATE_INDEX);
}
