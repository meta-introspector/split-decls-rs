use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The state of the lazy external source loading mechanism of a `SourceFile`.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ExternalSourceKind {
    /// The external source has been loaded already.
    Present(Arc<String>),
    /// No attempt has been made to load the external source.
    AbsentOk,
    /// A failed attempt has been made to load the external source.
    AbsentErr,
}
