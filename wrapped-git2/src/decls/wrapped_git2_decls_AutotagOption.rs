use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Automatic tag following options.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AutotagOption {
    /// Use the setting from the remote's configuration
    Unspecified,
    /// Ask the server for tags pointing to objects we're already downloading
    Auto,
    /// Don't ask for any tags beyond the refspecs
    None,
    /// Ask for all the tags
    All,
}
