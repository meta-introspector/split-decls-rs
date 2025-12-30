// Generated macro for PTHREAD_MUTEX_KIND_UNCHANGED (const)
macro_rules! Depcrate_shims_unix_syncPTHREAD_MUTEX_KIND_UNCHANGED {
() => {
// Module: crate::shims::unix::sync
// Provides: {"PTHREAD_MUTEX_KIND_UNCHANGED"}
// Dependencies: {}
# [doc = " To differentiate \"the mutex kind has not been changed\" from"] # [doc = " \"the mutex kind has been set to PTHREAD_MUTEX_DEFAULT and that is"] # [doc = " equal to some other mutex kind\", we make the default value of this"] # [doc = " field *not* PTHREAD_MUTEX_DEFAULT but this special flag."] const PTHREAD_MUTEX_KIND_UNCHANGED : i32 = 0x8000000 ;
};
}
