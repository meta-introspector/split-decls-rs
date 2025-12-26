use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub struct Interned<T: Internable + ?Sized> {
    arc: Arc<T>,
}
