use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Initialization vector (nonce) used by [`IvSizeUser`] implementors.
pub type Iv<B> = Array<u8, <B as IvSizeUser>::IvSize>;
