use serde::{Deserialize, Serialize};
use std::collections::HashMap;
trait UnwrapCapOverflow<T> {
    fn unwrap_cap_overflow(self) -> T;
}
