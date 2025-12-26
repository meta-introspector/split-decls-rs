use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, C> PartialEq<T> for OwnedEntry<T, C>
where
    T: PartialEq<T>,
    C: cfg::Config,
{
    fn eq(&self, other: &T) -> bool {
        *self.value() == *other
    }
}
