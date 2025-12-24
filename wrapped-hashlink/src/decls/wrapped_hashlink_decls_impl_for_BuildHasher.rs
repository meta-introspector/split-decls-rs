use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl BuildHasher for DefaultHashBuilder {
    type Hasher = DefaultHasher;
    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        DefaultHasher(self.0.build_hasher())
    }
}
