use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Clone for Rng {
    /// Clones the generator by creating a new generator with the same seed.
    fn clone(&self) -> Rng {
        Rng::with_seed(self.0)
    }
}
