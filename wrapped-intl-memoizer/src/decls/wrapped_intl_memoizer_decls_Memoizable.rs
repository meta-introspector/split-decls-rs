use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The trait that needs to be implemented for each intl formatter that needs to be
/// memoized.
pub trait Memoizable {
    /// Type of the arguments that are used to construct the formatter.
    type Args: 'static + Eq + Hash + Clone;
    /// Type of any errors that can occur during the construction process.
    type Error;
    /// Construct a formatter. This maps the [`Self::Args`] type to the actual constructor
    /// for an intl formatter.
    fn construct(lang: LanguageIdentifier, args: Self::Args) -> Result<Self, Self::Error>
    where
        Self: std::marker::Sized;
}
