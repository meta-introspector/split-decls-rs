use serde::{Deserialize, Serialize};
use std::collections::HashMap;
macro_rules! impl_is_terminal {
    ($($t:ty),*$(,) ?) => {
        $(impl sealed::Sealed for $t {} impl IsTerminal for $t { #[inline] fn
        is_terminal(& self) -> bool { std::io::IsTerminal::is_terminal(self) } })*
    };
}
