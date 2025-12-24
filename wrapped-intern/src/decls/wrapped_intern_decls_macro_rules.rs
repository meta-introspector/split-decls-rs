use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Implements `Internable` for a given list of types, making them usable with `Interned`.
#[macro_export]
#[doc(hidden)]
macro_rules! _impl_internable {
    ($($t:path),+ $(,)?) => {
        $(impl $crate::Internable for $t { fn storage() -> &'static $crate::InternStorage
        < Self > { static STORAGE : $crate::InternStorage <$t > =
        $crate::InternStorage::new(); & STORAGE } })+
    };
}
