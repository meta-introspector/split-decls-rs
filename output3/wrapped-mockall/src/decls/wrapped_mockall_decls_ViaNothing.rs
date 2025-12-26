use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[doc(hidden)]
pub trait ViaNothing {
    fn debug_string(&self) -> NothingPrint;
}
