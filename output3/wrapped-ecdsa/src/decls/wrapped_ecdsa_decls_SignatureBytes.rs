use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Fixed-size byte array containing an ECDSA signature
pub type SignatureBytes<C> = Array<u8, SignatureSize<C>>;
