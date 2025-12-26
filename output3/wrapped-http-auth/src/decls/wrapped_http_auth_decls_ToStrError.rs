use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An error returned by [`HeaderValue::to_str`].
pub struct ToStrError {
    _priv: (),
}
