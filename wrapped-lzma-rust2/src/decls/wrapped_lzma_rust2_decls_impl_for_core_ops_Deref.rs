use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: AutoFinish> core::ops::Deref for AutoFinisher<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}
