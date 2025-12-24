use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(
    not(
        any(
            target_pointer_width = "16",
            target_pointer_width = "32",
            target_pointer_width = "64"
        )
    )
)]
compile_error!("not supported on non-{16,32,64}, please file an issue");
