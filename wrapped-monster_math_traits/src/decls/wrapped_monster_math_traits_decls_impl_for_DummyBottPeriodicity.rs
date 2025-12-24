use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl DummyBottPeriodicity {
    pub fn new(period: String, phi_signature: u64, monster_element: u64) -> Self {
        Self {
            data: BottPeriodicityData {
                period,
                phi_signature,
                monster_element,
            },
        }
    }
}
