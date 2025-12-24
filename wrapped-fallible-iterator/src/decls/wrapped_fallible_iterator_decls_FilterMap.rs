use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator which both filters and maps the values of the underlying
/// iterator.
#[derive(Clone, Debug)]
pub struct FilterMap<I, F> {
    it: I,
    f: F,
}
