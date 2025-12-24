use serde::{Deserialize, Serialize};
use std::collections::HashMap;
digest::buffer_fixed!(
    #[doc = " MD5 hasher state."] pub struct Md5(block_api::Md5Core); oid :
    "1.2.840.113549.2.5"; impl : FixedHashTraits;
);
