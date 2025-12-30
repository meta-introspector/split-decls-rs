// Generated macro for access (module)
macro_rules! Depcrate_repository_thread_safeaccess {
() => {
// Module: crate::repository::thread_safe
// Provides: {"access"}
// Dependencies: {}
mod access { impl crate :: ThreadSafeRepository { # [doc = " Add thread-local state to an easy-to-use thread-local repository for the most convenient API."] pub fn to_thread_local (& self) -> crate :: Repository { self . into () } } }
};
}
