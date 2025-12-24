use serde::{Deserialize, Serialize};
use std::collections::HashMap;
unsafe impl<T: Sync, N: ArrayLength> Sync for GenericArray<T, N> {}
