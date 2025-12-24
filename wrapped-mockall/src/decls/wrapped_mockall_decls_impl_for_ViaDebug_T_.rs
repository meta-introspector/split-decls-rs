use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a, T: Debug> ViaDebug<T> for &ArgPrinter<'a, T> {
    fn debug_string(&self) -> DebugPrint<'a, T> {
        DebugPrint(self.0)
    }
}
