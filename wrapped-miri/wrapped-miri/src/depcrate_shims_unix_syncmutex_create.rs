// Generated macro for mutex_create (function)
macro_rules! Depcrate_shims_unix_syncmutex_create {
() => {
// Module: crate::shims::unix::sync
// Provides: {"mutex_create"}
// Dependencies: {}
# [doc = " Eagerly create and initialize a new mutex."] fn mutex_create < 'tcx > (ecx : & mut MiriInterpCx < 'tcx > , mutex_ptr : & OpTy < 'tcx > , kind : MutexKind ,) -> InterpResult < 'tcx , PthreadMutex > { let mutex = ecx . deref_pointer_as (mutex_ptr , ecx . libc_ty_layout ("pthread_mutex_t")) ? ; let data = PthreadMutex { mutex_ref : MutexRef :: new () , kind } ; ecx . lazy_sync_init (& mutex , mutex_init_offset (ecx) ? , data . clone ()) ? ; interp_ok (data) }
};
}
