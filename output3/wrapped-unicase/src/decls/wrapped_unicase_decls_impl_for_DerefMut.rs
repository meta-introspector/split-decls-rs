use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<S> DerefMut for UniCase<S> {
    #[inline]
    fn deref_mut<'a>(&'a mut self) -> &'a mut S {
        inner!(mut self.0)
    }
}
