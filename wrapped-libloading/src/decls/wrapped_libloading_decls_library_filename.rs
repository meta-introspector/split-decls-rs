use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Converts a library name to a filename generally appropriate for use on the system.
///
/// This function will prepend prefixes (such as `lib`) and suffixes (such as `.so`) to the library
/// `name` to construct the filename.
///
/// # Examples
///
/// It can be used to load global libraries in a platform independent manner:
///
/// ```
/// use libloading::{Library, library_filename};
/// // Will attempt to load `libLLVM.so` on Linux, `libLLVM.dylib` on macOS and `LLVM.dll` on
/// // Windows.
/// let library = unsafe {
///     Library::new(library_filename("LLVM"))
/// };
/// ```
#[cfg(feature = "std")]
#[cfg_attr(libloading_docs, doc(cfg(feature = "std")))]
pub fn library_filename<S: AsRef<std::ffi::OsStr>>(name: S) -> std::ffi::OsString {
    use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};
    let name = name.as_ref();
    let mut string =
        std::ffi::OsString::with_capacity(name.len() + DLL_PREFIX.len() + DLL_SUFFIX.len());
    string.push(DLL_PREFIX);
    string.push(name);
    string.push(DLL_SUFFIX);
    string
}
