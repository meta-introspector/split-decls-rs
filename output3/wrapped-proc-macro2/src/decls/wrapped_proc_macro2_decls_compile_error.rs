use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(
    procmacro2_nightly_testing,
    feature = "proc-macro",
    not(proc_macro_span)
))]
compile_error! {
    "\
    Build script probe failed to compile.
"
}
