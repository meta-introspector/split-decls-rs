use serde::{Deserialize, Serialize};
use std::collections::HashMap;
macro_rules! assert_capacity_limit_const {
    ($cap:expr) => {
        if std::mem::size_of::< usize > () > std::mem::size_of::< LenUint > () { if $cap
        > LenUint::MAX as usize { [] [$cap] } }
    };
}
