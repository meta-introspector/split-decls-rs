use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> Clone for IdxRange<T> {
    fn clone(&self) -> Self {
        Self {
            range: self.range.clone(),
            _p: PhantomData,
        }
    }
}
