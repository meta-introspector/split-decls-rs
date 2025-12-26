use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Credential process JSON protocol version.
///
/// If the protocol needs to make
/// a breaking change, a new protocol version should be defined (`PROTOCOL_VERSION_2`).
/// This library should offer support for both protocols if possible, by signaling
/// in the `CredentialHello` message. Cargo will then choose which protocol to use,
/// or it will error if there are no common protocol versions available.
pub const PROTOCOL_VERSION_1: u32 = 1;
