use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns the storage directory used by Cargo, often known as
/// `.cargo` or `CARGO_HOME`.
///
/// It returns one of the following values, in this order of
/// preference:
///
/// - The value of the `CARGO_HOME` environment variable, if it is
///   an absolute path.
/// - The value of the current working directory joined with the value
///   of the `CARGO_HOME` environment variable, if `CARGO_HOME` is a
///   relative directory.
/// - The `.cargo` directory in the user's home directory, as reported
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
/// match home::cargo_home() {
///     Ok(path) => println!("{}", path.display()),
///     Err(err) => eprintln!("Cannot get your cargo home dir: {:?}", err),
/// }
/// ```
pub fn cargo_home() -> io::Result<PathBuf> {
    env::cargo_home_with_env(&env::OS_ENV)
}
