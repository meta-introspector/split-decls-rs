use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Marks async function to be executed by runtime, suitable to test environment.
/// This macro helps set up a `Runtime` without requiring the user to use
/// [Runtime](../tokio/runtime/struct.Runtime.html) or
/// [Builder](../tokio/runtime/struct.Builder.html) directly.
///
/// Note: This macro is designed to be simplistic and targets applications that
/// do not require a complex setup. If the provided functionality is not
/// sufficient, you may be interested in using
/// [Builder](../tokio/runtime/struct.Builder.html), which provides a more
/// powerful interface.
///
/// # Multi-threaded runtime
///
/// To use the multi-threaded runtime, the macro can be configured using
///
/// ```no_run
/// #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
/// async fn my_test() {
///     assert!(true);
/// }
/// ```
///
/// The `worker_threads` option configures the number of worker threads, and
/// defaults to the number of cpus on the system.
///
/// Note: The multi-threaded runtime requires the `rt-multi-thread` feature
/// flag.
///
/// # Current thread runtime
///
/// The default test runtime is single-threaded. Each test gets a
/// separate current-thread runtime.
///
/// ```no_run
/// #[tokio::test]
/// async fn my_test() {
///     assert!(true);
/// }
/// ```
///
/// ## Usage
///
/// ### Using the multi-thread runtime
///
/// ```no_run
/// #[tokio::test(flavor = "multi_thread")]
/// async fn my_test() {
///     assert!(true);
/// }
/// ```
///
/// Equivalent code not using `#[tokio::test]`
///
/// ```no_run
/// #[test]
/// fn my_test() {
///     tokio::runtime::Builder::new_multi_thread()
///         .enable_all()
///         .build()
///         .unwrap()
///         .block_on(async {
///             assert!(true);
///         })
/// }
/// ```
///
/// ### Using current thread runtime
///
/// ```no_run
/// #[tokio::test]
/// async fn my_test() {
///     assert!(true);
/// }
/// ```
///
/// Equivalent code not using `#[tokio::test]`
///
/// ```no_run
/// #[test]
/// fn my_test() {
///     tokio::runtime::Builder::new_current_thread()
///         .enable_all()
///         .build()
///         .unwrap()
///         .block_on(async {
///             assert!(true);
///         })
/// }
/// ```
///
/// ### Set number of worker threads
///
/// ```no_run
/// #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
/// async fn my_test() {
///     assert!(true);
/// }
/// ```
///
/// Equivalent code not using `#[tokio::test]`
///
/// ```no_run
/// #[test]
/// fn my_test() {
///     tokio::runtime::Builder::new_multi_thread()
///         .worker_threads(2)
///         .enable_all()
///         .build()
///         .unwrap()
///         .block_on(async {
///             assert!(true);
///         })
/// }
/// ```
///
/// ### Configure the runtime to start with time paused
///
/// ```no_run
/// #[tokio::test(start_paused = true)]
/// async fn my_test() {
///     assert!(true);
/// }
/// ```
///
/// Equivalent code not using `#[tokio::test]`
///
/// ```no_run
/// #[test]
/// fn my_test() {
///     tokio::runtime::Builder::new_current_thread()
///         .enable_all()
///         .start_paused(true)
///         .build()
///         .unwrap()
///         .block_on(async {
///             assert!(true);
///         })
/// }
/// ```
///
/// Note that `start_paused` requires the `test-util` feature to be enabled.
///
/// ### Rename package
///
/// ```rust
/// use tokio as tokio1;
///
/// #[tokio1::test(crate = "tokio1")]
/// async fn my_test() {
///     println!("Hello world");
/// }
/// ```
///
/// ### Configure unhandled panic behavior
///
/// Available options are `shutdown_runtime` and `ignore`. For more details, see
/// [`Builder::unhandled_panic`].
///
/// This option is only compatible with the `current_thread` runtime.
///
/// ```no_run
/// #[cfg(tokio_unstable)]
/// #[tokio::test(flavor = "current_thread", unhandled_panic = "shutdown_runtime")]
/// async fn my_test() {
///     let _ = tokio::spawn(async {
///         panic!("This panic will shutdown the runtime.");
///     }).await;
/// }
///
/// # fn main() { }
/// ```
///
/// Equivalent code not using `#[tokio::test]`
///
/// ```no_run
/// #[cfg(tokio_unstable)]
/// #[test]
/// fn my_test() {
///     tokio::runtime::Builder::new_current_thread()
///         .enable_all()
///         .unhandled_panic(UnhandledPanic::ShutdownRuntime)
///         .build()
///         .unwrap()
///         .block_on(async {
///             let _ = tokio::spawn(async {
///                 panic!("This panic will shutdown the runtime.");
///             }).await;
///         })
/// }
///
/// # fn main() { }
/// ```
///
/// **Note**: This option depends on Tokio's [unstable API][unstable]. See [the
/// documentation on unstable features][unstable] for details on how to enable
/// Tokio's unstable features.
///
/// [`Builder::unhandled_panic`]: ../tokio/runtime/struct.Builder.html#method.unhandled_panic
/// [unstable]: ../tokio/index.html#unstable-features
#[proc_macro_attribute]
pub fn test(args: TokenStream, item: TokenStream) -> TokenStream {
    entry::test(args.into(), item.into(), true).into()
}
