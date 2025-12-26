use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: AutoFinish> core::ops::DerefMut for AutoFinisher<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}
