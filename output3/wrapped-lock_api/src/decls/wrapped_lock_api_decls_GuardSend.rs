use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Marker type which indicates that the Guard type for a lock is `Send`.
pub struct GuardSend(());
