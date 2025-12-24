use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod prelude {
    pub use crate::ArgLineCommandExt;
    pub use crate::ChannelChangerCommandExt;
    pub use crate::TestEnvCommandExt;
    pub use crate::cargo_test;
    pub use crate::paths::CargoPathExt;
    pub use snapbox::IntoData;
}
