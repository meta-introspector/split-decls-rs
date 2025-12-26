use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[macro_export]
macro_rules! fixme {
    ($msg:literal) => {{
        use patch_build_rs_macros::extract;
        extract!($msg);
        compile_error!(concat!("FIXME: ", $msg));
    }};
}
