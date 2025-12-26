use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Information about a specific binding. This contains both an `Ident`
/// reference to the given field, and the syn `&'a Field` descriptor for that
/// field.
///
/// This type supports `quote::ToTokens`, so can be directly used within the
/// `quote!` macro. It expands to a reference to the matched field.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BindingInfo<'a> {
    /// The name which this `BindingInfo` will bind to.
    pub binding: Ident,
    /// The type of binding which this `BindingInfo` will create.
    pub style: BindStyle,
    field: &'a Field,
    generics: &'a Generics,
    seen_generics: Vec<bool>,
    index: usize,
}
