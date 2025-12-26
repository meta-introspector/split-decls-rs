use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub trait HasDataLayout {
    fn data_layout(&self) -> &TargetDataLayout;
}
