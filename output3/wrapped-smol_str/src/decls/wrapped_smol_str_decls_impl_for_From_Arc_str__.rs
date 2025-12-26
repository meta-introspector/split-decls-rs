use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<Arc<str>> for SmolStr {
    #[inline]
    fn from(s: Arc<str>) -> SmolStr {
        let repr = Repr::new_on_stack(s.as_ref()).unwrap_or(Repr::Heap(s));
        Self(repr)
    }
}
