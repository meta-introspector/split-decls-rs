use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The `Error` type, a wrapper around a dynamic error type.
///
/// `Error` works a lot like `Box<dyn std::error::Error>`, but with these
/// differences:
///
/// - `Error` requires that the error is `Send`, `Sync`, and `'static`.
/// - `Error` guarantees that a backtrace is available, even if the underlying
///   error type does not provide one.
/// - `Error` is represented as a narrow pointer &mdash; exactly one word in
///   size instead of two.
///
/// <br>
///
/// # Display representations
///
/// When you print an error object using "{}" or to_string(), only the outermost
/// underlying error or context is printed, not any of the lower level causes.
/// This is exactly as if you had called the Display impl of the error from
/// which you constructed your anyhow::Error.
///
/// ```console
/// Failed to read instrs from ./path/to/instrs.json
/// ```
///
/// To print causes as well using anyhow's default formatting of causes, use the
/// alternate selector "{:#}".
///
/// ```console
/// Failed to read instrs from ./path/to/instrs.json: No such file or directory (os error 2)
/// ```
///
/// The Debug format "{:?}" includes your backtrace if one was captured. Note
/// that this is the representation you get by default if you return an error
/// from `fn main` instead of printing it explicitly yourself.
///
/// ```console
/// Error: Failed to read instrs from ./path/to/instrs.json
///
/// Caused by:
///     No such file or directory (os error 2)
/// ```
///
/// and if there is a backtrace available:
///
/// ```console
/// Error: Failed to read instrs from ./path/to/instrs.json
///
/// Caused by:
///     No such file or directory (os error 2)
///
/// Stack backtrace:
///    0: <E as anyhow::context::ext::StdError>::ext_context
///              at /git/anyhow/src/backtrace.rs:26
///    1: core::result::Result<T,E>::map_err
///              at /git/rustc/src/libcore/result.rs:596
///    2: anyhow::context::<impl anyhow::Context<T,E> for core::result::Result<T,E>>::with_context
///              at /git/anyhow/src/context.rs:58
///    3: testing::main
///              at src/main.rs:5
///    4: std::rt::lang_start
///              at /git/rustc/src/libstd/rt.rs:61
///    5: main
///    6: __libc_start_main
///    7: _start
/// ```
///
/// To see a conventional struct-style Debug representation, use "{:#?}".
///
/// ```console
/// Error {
///     context: "Failed to read instrs from ./path/to/instrs.json",
///     source: Os {
///         code: 2,
///         kind: NotFound,
///         message: "No such file or directory",
///     },
/// }
/// ```
///
/// If none of the built-in representations are appropriate and you would prefer
/// to render the error and its cause chain yourself, it can be done something
/// like this:
///
/// ```
/// use anyhow::{Context, Result};
///
/// fn main() {
///     if let Err(err) = try_main() {
///         eprintln!("ERROR: {}", err);
///         err.chain().skip(1).for_each(|cause| eprintln!("because: {}", cause));
///         std::process::exit(1);
///     }
/// }
///
/// fn try_main() -> Result<()> {
///     # const IGNORE: &str = stringify! {
///     ...
///     # };
///     # Ok(())
/// }
/// ```
#[repr(transparent)]
pub struct Error {
    inner: Own<ErrorImpl>,
}
