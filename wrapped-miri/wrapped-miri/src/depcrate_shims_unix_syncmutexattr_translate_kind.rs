// Generated macro for mutexattr_translate_kind (function)
macro_rules! Depcrate_shims_unix_syncmutexattr_translate_kind {
() => {
// Module: crate::shims::unix::sync
// Provides: {"mutexattr_translate_kind"}
// Dependencies: {}
# [doc = " Translates the mutex kind from what is stored in pthread_mutexattr_t to our enum."] fn mutexattr_translate_kind < 'tcx > (ecx : & MiriInterpCx < 'tcx > , kind : i32 ,) -> InterpResult < 'tcx , MutexKind > { interp_ok (if kind == (ecx . eval_libc_i32 ("PTHREAD_MUTEX_NORMAL")) { MutexKind :: Normal } else if kind == ecx . eval_libc_i32 ("PTHREAD_MUTEX_ERRORCHECK") { MutexKind :: ErrorCheck } else if kind == ecx . eval_libc_i32 ("PTHREAD_MUTEX_RECURSIVE") { MutexKind :: Recursive } else if kind == ecx . eval_libc_i32 ("PTHREAD_MUTEX_DEFAULT") || kind == PTHREAD_MUTEX_KIND_UNCHANGED { MutexKind :: Default } else { throw_unsup_format ! ("unsupported type of mutex: {kind}") ; }) }
};
}
