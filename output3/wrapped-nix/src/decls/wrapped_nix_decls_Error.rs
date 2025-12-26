use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Nix's main error type.
///
/// It's a wrapper around Errno.  As such, it's very interoperable with
/// [`std::io::Error`], but it has the advantages of:
/// * `Clone`
/// * `Copy`
/// * `Eq`
/// * Small size
/// * Represents all of the system's errnos, instead of just the most common
///   ones.
pub type Error = Errno;
