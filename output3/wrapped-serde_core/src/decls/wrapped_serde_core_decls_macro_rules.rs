use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[macro_export]
#[doc(hidden)]
macro_rules! __require_serde_not_serde_core {
    () => {
        ::core::compile_error!(
            "Serde derive requires a dependency on the serde crate, not serde_core"
        );
    };
}
