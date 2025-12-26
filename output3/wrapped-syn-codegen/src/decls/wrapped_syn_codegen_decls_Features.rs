use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Features behind which a syntax tree type is cfg gated.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Features {
    /// Type is accessible if at least one of these features is enabled against
    /// the Syn dependency.
    pub any: BTreeSet<String>,
}
