// Generated macro for ensure_safe_sqlite_threading_mode (function)
macro_rules! Depcrate_inner_connectionensure_safe_sqlite_threading_mode {
() => {
// Module: crate::inner_connection
// Provides: {"ensure_safe_sqlite_threading_mode"}
// Dependencies: {}
# [cfg (not (any (target_arch = "wasm32")))] fn ensure_safe_sqlite_threading_mode () -> Result < () > { if unsafe { ffi :: sqlite3_threadsafe () == 0 } { return Err (Error :: SqliteSingleThreadedMode) ; } const SQLITE_SINGLETHREADED_MUTEX_MAGIC : usize = 8 ; let is_singlethreaded = unsafe { let mutex_ptr = ffi :: sqlite3_mutex_alloc (0) ; let is_singlethreaded = mutex_ptr as usize == SQLITE_SINGLETHREADED_MUTEX_MAGIC ; ffi :: sqlite3_mutex_free (mutex_ptr) ; is_singlethreaded } ; if is_singlethreaded { Err (Error :: SqliteSingleThreadedMode) } else { Ok (()) } }
};
}
