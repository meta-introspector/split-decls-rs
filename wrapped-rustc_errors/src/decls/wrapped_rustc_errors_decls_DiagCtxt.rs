use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A `DiagCtxt` deals with errors and other compiler output.
/// Certain errors (fatal, bug, unimpl) may cause immediate exit,
/// others log errors for later reporting.
pub struct DiagCtxt {
    inner: Lock<DiagCtxtInner>,
}
