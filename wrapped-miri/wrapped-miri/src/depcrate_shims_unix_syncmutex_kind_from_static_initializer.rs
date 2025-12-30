// Generated macro for mutex_kind_from_static_initializer (function)
macro_rules! Depcrate_shims_unix_syncmutex_kind_from_static_initializer {
() => {
// Module: crate::shims::unix::sync
// Provides: {"mutex_kind_from_static_initializer"}
// Dependencies: {}
# [doc = " Returns the kind of a static initializer."] fn mutex_kind_from_static_initializer < 'tcx > (ecx : & MiriInterpCx < 'tcx > , mutex : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , MutexKind > { let is_initializer = | name | bytewise_equal_atomic_relaxed (ecx , mutex , & ecx . eval_path (& ["libc" , name])) ; if is_initializer ("PTHREAD_MUTEX_INITIALIZER") ? { return interp_ok (MutexKind :: Default) ; } match & * ecx . tcx . sess . target . os { "linux" => if is_initializer ("PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP") ? { return interp_ok (MutexKind :: Recursive) ; } else if is_initializer ("PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP") ? { return interp_ok (MutexKind :: ErrorCheck) ; } , _ => { } } throw_unsup_format ! ("unsupported static initializer used for `pthread_mutex_t`") ; }
};
}
