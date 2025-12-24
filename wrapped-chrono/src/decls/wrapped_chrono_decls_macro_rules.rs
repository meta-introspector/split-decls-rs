use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Workaround because `?` is not (yet) available in const context.
#[macro_export]
#[doc(hidden)]
macro_rules! try_opt {
    ($e:expr) => {
        match $e { Some(v) => v, None => return None, }
    };
}
