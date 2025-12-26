use serde::{Deserialize, Serialize};
use std::collections::HashMap;
feature! {
    #![all(feature = "registry", feature = "std")] pub use registry::Registry; #[doc =
    " Returns a default [`Registry`]."] pub fn registry() -> Registry {
    Registry::default() }
}
