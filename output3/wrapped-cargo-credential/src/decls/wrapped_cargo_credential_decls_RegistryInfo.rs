use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RegistryInfo<'a> {
    /// Registry index url
    pub index_url: &'a str,
    /// Name of the registry in configuration. May not be available.
    /// The crates.io registry will be `crates-io` (`CRATES_IO_REGISTRY`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Headers from attempting to access a registry that resulted in a HTTP 401.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub headers: Vec<String>,
}
