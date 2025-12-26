use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Unwrap a `Result` with a useful panic message
///
/// # Example
///
/// ```rust
/// use cargo_test_support::t;
/// t!(std::fs::read_to_string("Cargo.toml"));
/// ```
#[macro_export]
macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => $crate::panic_error(&format!("failed running {}", stringify!($e)), e),
        }
    };
}
