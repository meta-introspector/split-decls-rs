use serde::{Deserialize, Serialize};
use std::collections::HashMap;
macro_rules! if_hyper {
    ($($item:item)*) => {
        $(#[cfg(not(target_arch = "wasm32"))] $item)*
    };
}
