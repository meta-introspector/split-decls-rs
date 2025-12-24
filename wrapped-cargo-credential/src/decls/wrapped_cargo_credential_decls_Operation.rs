use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A record of what kind of operation is happening that we should generate a token for.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[serde(tag = "operation", rename_all = "kebab-case")]
pub enum Operation<'a> {
    /// The user is attempting to fetch a crate.
    Read,
    /// The user is attempting to publish a crate.
    Publish {
        /// The name of the crate
        name: &'a str,
        /// The version of the crate
        vers: &'a str,
        /// The checksum of the crate file being uploaded
        cksum: &'a str,
    },
    /// The user is attempting to yank a crate.
    Yank {
        /// The name of the crate
        name: &'a str,
        /// The version of the crate
        vers: &'a str,
    },
    /// The user is attempting to unyank a crate.
    Unyank {
        /// The name of the crate
        name: &'a str,
        /// The version of the crate
        vers: &'a str,
    },
    /// The user is attempting to modify the owners of a crate.
    Owners {
        /// The name of the crate
        name: &'a str,
    },
    #[serde(other)]
    Unknown,
}
