// Generated macro for simplified (function)
macro_rules! Depcratesimplified {
() => {
// Module: crate
// Provides: {"simplified"}
// Dependencies: {}
# [doc = " Takes any path, and when possible, converts Windows UNC paths to regular paths."] # [doc = " If the path can't be converted, it's returned unmodified."] # [doc = ""] # [doc = " On non-Windows this is no-op."] # [doc = ""] # [doc = " `\\\\?\\C:\\Windows` will be converted to `C:\\Windows`,"] # [doc = " but `\\\\?\\C:\\COM` will be left as-is (due to a reserved filename)."] # [doc = ""] # [doc = " Use this to pass arbitrary paths to programs that may not be UNC-aware."] # [doc = ""] # [doc = " It's generally safe to pass UNC paths to legacy programs, because"] # [doc = " these paths contain a reserved prefix, so will gracefully fail"] # [doc = " if used with legacy APIs that don't support UNC."] # [doc = ""] # [doc = " This function does not perform any I/O."] # [doc = ""] # [doc = " Currently paths with unpaired surrogates aren't converted even if they"] # [doc = " could be, due to limitations of Rust's `OsStr` API."] # [doc = ""] # [doc = " To check if a path remained as UNC, use [`is_simplified()`] or `path.as_os_str().as_encoded_bytes().starts_with(b\"\\\\\\\\\")`."] # [inline] # [must_use] pub fn simplified (path : & Path) -> & Path { try_simplified (path) . unwrap_or (path) }
};
}
