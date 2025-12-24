use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A delimited token stream.
///
/// A `Group` internally contains a `TokenStream` which is surrounded by
/// `Delimiter`s.
#[derive(Clone)]
pub struct Group {
    inner: imp::Group,
}
