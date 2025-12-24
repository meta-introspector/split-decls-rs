use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait for loading current IV state.
pub trait IvState: IvSizeUser {
    /// Returns current IV state.
    fn iv_state(&self) -> Iv<Self>;
}
