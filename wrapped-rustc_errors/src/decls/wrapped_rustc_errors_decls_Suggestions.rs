use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Represents the help messages seen on a diagnostic.
#[derive(Clone, Debug, PartialEq, Hash, Encodable, Decodable)]
pub enum Suggestions {
    /// Indicates that new suggestions can be added or removed from this diagnostic.
    ///
    /// `DiagInner`'s new_* methods initialize the `suggestions` field with
    /// this variant. Also, this is the default variant for `Suggestions`.
    Enabled(Vec<CodeSuggestion>),
    /// Indicates that suggestions cannot be added or removed from this diagnostic.
    ///
    /// Gets toggled when `.seal_suggestions()` is called on the `DiagInner`.
    Sealed(Box<[CodeSuggestion]>),
    /// Indicates that no suggestion is available for this diagnostic.
    ///
    /// Gets toggled when `.disable_suggestions()` is called on the `DiagInner`.
    Disabled,
}
