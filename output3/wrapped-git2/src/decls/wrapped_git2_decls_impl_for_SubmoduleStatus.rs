use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl SubmoduleStatus {
    is_bit_set!(is_in_head, SubmoduleStatus::IN_HEAD);
    is_bit_set!(is_in_index, SubmoduleStatus::IN_INDEX);
    is_bit_set!(is_in_config, SubmoduleStatus::IN_CONFIG);
    is_bit_set!(is_in_wd, SubmoduleStatus::IN_WD);
    is_bit_set!(is_index_added, SubmoduleStatus::INDEX_ADDED);
    is_bit_set!(is_index_deleted, SubmoduleStatus::INDEX_DELETED);
    is_bit_set!(is_index_modified, SubmoduleStatus::INDEX_MODIFIED);
    is_bit_set!(is_wd_uninitialized, SubmoduleStatus::WD_UNINITIALIZED);
    is_bit_set!(is_wd_added, SubmoduleStatus::WD_ADDED);
    is_bit_set!(is_wd_deleted, SubmoduleStatus::WD_DELETED);
    is_bit_set!(is_wd_modified, SubmoduleStatus::WD_MODIFIED);
    is_bit_set!(is_wd_wd_modified, SubmoduleStatus::WD_WD_MODIFIED);
    is_bit_set!(is_wd_untracked, SubmoduleStatus::WD_UNTRACKED);
}
