use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub enum AssocItemContainer {
    Trait(Trait),
    Impl(Impl),
}
