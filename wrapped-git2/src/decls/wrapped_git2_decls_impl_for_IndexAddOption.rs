use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl IndexAddOption {
    is_bit_set!(is_default, IndexAddOption::DEFAULT);
    is_bit_set!(is_force, IndexAddOption::FORCE);
    is_bit_set!(is_disable_pathspec_match, IndexAddOption::DISABLE_PATHSPEC_MATCH);
    is_bit_set!(is_check_pathspec, IndexAddOption::CHECK_PATHSPEC);
}
