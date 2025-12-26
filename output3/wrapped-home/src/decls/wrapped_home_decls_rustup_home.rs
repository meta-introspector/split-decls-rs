use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns the storage directory used by rustup, often known as
/// `.rustup` or `RUSTUP_HOME`.
///
/// It returns one of the following values, in this order of
/// preference:
///
/// - The value of the `RUSTUP_HOME` environment variable, if it is
///   an absolute path.
/// - The value of the current working directory joined with the value
///   of the `RUSTUP_HOME` environment variable, if `RUSTUP_HOME` is a
///   relative directory.
/// - The `.rustup` directory in the user's home directory, as reported
///   by the `home_dir` function.
///
/// # Errors
///
/// This function fails if it fails to retrieve the current directory,
/// or if the home directory cannot be determined.
///
/// # Examples
///
/// ```
/// match home::rustup_home() {
///     Ok(path) => println!("{}", path.display()),
///     Err(err) => eprintln!("Cannot get your rustup home dir: {:?}", err),
/// }
/// ```
pub fn rustup_home() -> io::Result<PathBuf> {
    env::rustup_home_with_env(&env::OS_ENV)
}
