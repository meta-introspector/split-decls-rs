use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> AsMut<[T]> for Arena<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.data.as_mut()
    }
}
