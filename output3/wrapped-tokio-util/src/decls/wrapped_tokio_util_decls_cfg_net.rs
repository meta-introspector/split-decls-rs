use serde::{Deserialize, Serialize};
use std::collections::HashMap;
cfg_net! {
    #[cfg(not(target_arch = "wasm32"))] pub mod udp; pub mod net;
}
