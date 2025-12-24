use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl MonsterConstants for DummyMonsterConstants {
    fn get_representation_dimension(&self) -> u32 {
        196883
    }
    fn get_order_str(&self) -> &str {
        "808017424794512875886459904961710757005754368000000000"
    }
    fn get_supersingular_prime_factors_count(&self) -> u32 {
        108
    }
}
