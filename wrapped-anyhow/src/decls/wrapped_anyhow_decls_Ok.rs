use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Equivalent to `Ok::<_, anyhow::Error>(value)`.
///
/// This simplifies creation of an `anyhow::Result` in places where type
/// inference cannot deduce the `E` type of the result &mdash; without needing
/// to write `Ok::<_, anyhow::Error>(value)`.
///
/// One might think that `anyhow::Result::Ok(value)` would work in such cases
/// but it does not.
///
/// ```console
/// error[E0282]: type annotations needed for `std::result::Result<i32, E>`
///   --> src/main.rs:11:13
///    |
/// 11 |     let _ = anyhow::Result::Ok(1);
///    |         -   ^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `E` declared on the enum `Result`
///    |         |
///    |         consider giving this pattern the explicit type `std::result::Result<i32, E>`, where the type parameter `E` is specified
/// ```
#[allow(non_snake_case)]
pub fn Ok<T>(value: T) -> Result<T> {
    Result::Ok(value)
}
