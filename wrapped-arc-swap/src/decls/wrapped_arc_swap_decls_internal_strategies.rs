use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(feature = "internal-test-strategies", test))]
#[allow(deprecated)]
mod internal_strategies {
    use super::*;
    t!(tests_full_slots, crate ::strategy::test_strategies::FillFastSlots);
}
