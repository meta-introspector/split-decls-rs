use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Status {
    is_bit_set!(is_index_new, Status::INDEX_NEW);
    is_bit_set!(is_index_modified, Status::INDEX_MODIFIED);
    is_bit_set!(is_index_deleted, Status::INDEX_DELETED);
    is_bit_set!(is_index_renamed, Status::INDEX_RENAMED);
    is_bit_set!(is_index_typechange, Status::INDEX_TYPECHANGE);
    is_bit_set!(is_wt_new, Status::WT_NEW);
    is_bit_set!(is_wt_modified, Status::WT_MODIFIED);
    is_bit_set!(is_wt_deleted, Status::WT_DELETED);
    is_bit_set!(is_wt_typechange, Status::WT_TYPECHANGE);
    is_bit_set!(is_wt_renamed, Status::WT_RENAMED);
    is_bit_set!(is_ignored, Status::IGNORED);
    is_bit_set!(is_conflicted, Status::CONFLICTED);
}
