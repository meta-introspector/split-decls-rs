use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Variants of an enum syntax tree node.
///
/// The keys in the map are the variant names.
///
/// Variants are unit variants if they hold no data and tuple variants
/// otherwise. The Syn syntax tree does not make use of braced variants.
pub type Variants = IndexMap<String, Vec<Type>>;
