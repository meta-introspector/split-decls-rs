use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl_specific_ref_and_mut!(
    ::std::ffi::CStr, cfg(feature = "std"), doc = "Requires crate feature `std`."
);
