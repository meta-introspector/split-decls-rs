use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl PathspecFlags {
    is_bit_set!(is_default, PathspecFlags::DEFAULT);
    is_bit_set!(is_ignore_case, PathspecFlags::IGNORE_CASE);
    is_bit_set!(is_use_case, PathspecFlags::USE_CASE);
    is_bit_set!(is_no_glob, PathspecFlags::NO_GLOB);
    is_bit_set!(is_no_match_error, PathspecFlags::NO_MATCH_ERROR);
    is_bit_set!(is_find_failures, PathspecFlags::FIND_FAILURES);
    is_bit_set!(is_failures_only, PathspecFlags::FAILURES_ONLY);
}
