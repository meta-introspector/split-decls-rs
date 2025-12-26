use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns the path of the current user's home directory using environment
/// variables or OS-specific APIs.
///
/// # Unix
///
/// Returns the value of the `HOME` environment variable if it is set
/// **even** if it is an empty string. Otherwise, it tries to determine the
/// home directory by invoking the [`getpwuid_r`][getpwuid] function with
/// the UID of the current user.
///
/// [getpwuid]: https://linux.die.net/man/3/getpwuid_r
///
/// # Windows
///
/// Returns the value of the `USERPROFILE` environment variable if it is set
/// **and** it is not an empty string. Otherwise, it tries to determine the
/// home directory by invoking the [`SHGetKnownFolderPath`][shgkfp] function with
/// [`FOLDERID_Profile`][knownfolderid].
///
/// [shgkfp]: https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath
/// [knownfolderid]: https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid
///
/// # Examples
///
/// ```
/// match home::home_dir() {
///     Some(path) if !path.as_os_str().is_empty() => println!("{}", path.display()),
///     _ => println!("Unable to get your home dir!"),
/// }
/// ```
pub fn home_dir() -> Option<PathBuf> {
    env::home_dir_with_env(&env::OS_ENV)
}
