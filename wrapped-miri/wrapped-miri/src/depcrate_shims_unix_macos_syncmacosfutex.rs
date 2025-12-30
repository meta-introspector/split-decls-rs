// Generated macro for MacOsFutex (struct)
macro_rules! Depcrate_shims_unix_macos_syncMacOsFutex {
() => {
// Module: crate::shims::unix::macos::sync
// Provides: {"MacOsFutex"}
// Dependencies: {}
# [doc = " Metadata for a macOS futex."] # [doc = ""] # [doc = " Since macOS 11.0, Apple has exposed the previously private futex API consisting"] # [doc = " of `os_sync_wait_on_address` (and friends) and `os_sync_wake_by_address_{any, all}`."] # [doc = " These work with different value sizes and flags, which are validated to be consistent."] # [doc = " This structure keeps track of both the futex queue and these values."] struct MacOsFutex { futex : FutexRef , # [doc = " The size in bytes of the atomic primitive underlying this futex."] size : Cell < u64 > , # [doc = " Whether the futex is shared across process boundaries."] shared : Cell < bool > , }
};
}
