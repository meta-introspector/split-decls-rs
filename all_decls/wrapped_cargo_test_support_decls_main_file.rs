use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Generate a `main.rs` printing the specified text
///
/// ```rust
/// # use cargo_test_support::main_file;
/// # mod dep {
/// #     fn bar() -> &'static str {
/// #         "world"
/// #     }
/// # }
/// main_file(
///     r#""hello {}", dep::bar()"#,
///     &[]
/// );
/// ```
pub fn main_file(println: &str, externed_deps: &[&str]) -> String {
    let mut buf = String::new();
    for dep in externed_deps.iter() {
        buf.push_str(&format!("extern crate {};\n", dep));
    }
    buf.push_str("fn main() { println!(");
    buf.push_str(println);
    buf.push_str("); }\n");
    buf
}
