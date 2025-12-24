use serde::{Deserialize, Serialize};
use std::collections::HashMap;
cfg_default! {
    #[cfg(not(target_os = "unknown"))] pub mod fs; pub mod path; pub mod net;
    #[cfg(not(target_os = "unknown"))] pub (crate) mod rt;
}
