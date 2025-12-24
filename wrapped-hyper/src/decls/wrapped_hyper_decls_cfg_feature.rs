use serde::{Deserialize, Serialize};
use std::collections::HashMap;
cfg_feature! {
    #![feature = "server"] pub mod server;
}
