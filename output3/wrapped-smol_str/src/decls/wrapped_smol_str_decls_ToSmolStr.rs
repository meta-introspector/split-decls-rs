use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Convert value to [`SmolStr`] using [`fmt::Display`], potentially without allocating.
///
/// Almost identical to [`ToString`], but converts to `SmolStr` instead.
pub trait ToSmolStr {
    fn to_smolstr(&self) -> SmolStr;
}
