use serde::{Deserialize, Serialize};
use std::collections::HashMap;
cfg_if::cfg_if! {
    if #[cfg(feature = "std")] { mod hash_map; mod hash_set; pub use crate
    ::hash_map::AHashMap; pub use crate ::hash_set::AHashSet; #[doc =
    " [Hasher]: std::hash::Hasher"] #[doc = " [HashMap]: std::collections::HashMap"]
    #[doc = " Type alias for [HashMap]<K, V, ahash::RandomState>"] pub type HashMap < K,
    V > = std::collections::HashMap < K, V, crate ::RandomState >; #[doc =
    " Type alias for [HashSet]<K, ahash::RandomState>"] pub type HashSet < K > =
    std::collections::HashSet < K, crate ::RandomState >; }
}
