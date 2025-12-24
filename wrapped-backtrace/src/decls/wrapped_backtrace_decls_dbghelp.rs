use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(
    all(
        windows,
        any(
            target_env = "msvc",
            all(target_env = "gnu", any(target_arch = "x86", target_arch = "arm"))
        ),
        not(target_vendor = "uwp")
    )
)]
mod dbghelp;
