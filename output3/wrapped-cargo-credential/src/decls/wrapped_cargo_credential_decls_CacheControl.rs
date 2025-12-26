use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "cache", rename_all = "kebab-case")]
#[non_exhaustive]
pub enum CacheControl {
    /// Do not cache this result.
    Never,
    /// Cache this result and use it for subsequent requests in the current Cargo invocation until the specified time.
    Expires {
        #[serde(with = "time::serde::timestamp")]
        expiration: OffsetDateTime,
    },
    /// Cache this result and use it for all subsequent requests in the current Cargo invocation.
    Session,
    #[serde(other)]
    Unknown,
}
