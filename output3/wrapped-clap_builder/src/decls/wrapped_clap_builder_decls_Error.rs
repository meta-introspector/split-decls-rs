use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Command Line Argument Parser Error
///
/// See [`Command::error`] to create an error.
///
/// [`Command::error`]: crate::Command::error
pub type Error = error::Error<error::DefaultFormatter>;
