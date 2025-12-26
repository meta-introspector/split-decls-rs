use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Content of a syntax tree data structure.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Data {
    /// This is an opaque type with no publicly accessible structure.
    Private,
    /// This type is a braced struct with named fields.
    #[cfg_attr(feature = "serde", serde(rename = "fields"))]
    Struct(Fields),
    /// This type is an enum.
    #[cfg_attr(feature = "serde", serde(rename = "variants"))]
    Enum(Variants),
}
