use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Algorithm {
    #[cfg(test)]
    const ALL: [Self; 2] = [Algorithm::Histogram, Algorithm::Myers];
}
