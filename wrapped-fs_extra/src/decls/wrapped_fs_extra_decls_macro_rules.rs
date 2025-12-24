use serde::{Deserialize, Serialize};
use std::collections::HashMap;
macro_rules! err {
    ($text:expr, $kind:expr) => {
        return Err(Error::new($kind, $text))
    };
    ($text:expr) => {
        err!($text, ErrorKind::Other)
    };
}
