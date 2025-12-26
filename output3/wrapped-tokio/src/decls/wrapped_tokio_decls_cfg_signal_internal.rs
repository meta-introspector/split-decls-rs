use serde::{Deserialize, Serialize};
use std::collections::HashMap;
cfg_signal_internal! {
    #[cfg(not(feature = "signal"))] #[allow(dead_code)] #[allow(unreachable_pub)] pub
    (crate) mod signal;
}
