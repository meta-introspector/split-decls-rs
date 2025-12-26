use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: RefCnt, S: Strategy<T>> Deref for Guard<T, S> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        self.inner.borrow()
    }
}
