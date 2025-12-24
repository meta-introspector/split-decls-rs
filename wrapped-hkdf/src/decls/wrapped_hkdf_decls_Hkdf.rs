use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// [`GenericHkdf`] variant which uses [`Hmac`] for the underlying HMAC implementation.
pub type Hkdf<H> = GenericHkdf<Hmac<H>>;
