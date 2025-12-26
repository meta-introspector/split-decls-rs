use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Allows for the creation of parallel Rust tests that won't clash with serial tests
/// ````no_run
/// #[test]
/// #[serial]
/// fn test_serial_one() {
///   // Do things
/// }
///
/// #[test]
/// #[parallel]
/// fn test_parallel_one() {
///   // Do things
/// }
///
/// #[test]
/// #[parallel]
/// fn test_parallel_two() {
///   // Do things
/// }
/// ````
/// Multiple tests with the [parallel](macro@parallel) attribute may run in parallel, but not at the
/// same time as [serial](macro@serial) tests. e.g. in the example code above, `test_parallel_one`
/// and `test_parallel_two` may run at the same time, but `test_serial_one` is guaranteed not to run
/// at the same time as either of them. [parallel](macro@parallel) also takes key arguments for groups
/// of tests as per [serial](macro@serial).
///
/// Note that this has zero effect on [file_serial](macro@file_serial) tests, as that uses a different
/// serialisation mechanism. For that, you want [file_parallel](macro@file_parallel).
#[proc_macro_attribute]
pub fn parallel(attr: TokenStream, input: TokenStream) -> TokenStream {
    local_parallel_core(attr.into(), input.into()).into()
}
