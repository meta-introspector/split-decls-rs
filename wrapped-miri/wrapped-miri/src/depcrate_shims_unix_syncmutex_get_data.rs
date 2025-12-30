// Generated macro for mutex_get_data (function)
macro_rules! Depcrate_shims_unix_syncmutex_get_data {
() => {
// Module: crate::shims::unix::sync
// Provides: {"mutex_get_data"}
// Dependencies: {}
# [doc = " Returns the mutex data stored at the address that `mutex_ptr` points to."] # [doc = " Will raise an error if the mutex has been moved since its first use."] fn mutex_get_data < 'tcx , 'a > (ecx : & 'a mut MiriInterpCx < 'tcx > , mutex_ptr : & OpTy < 'tcx > ,) -> InterpResult < 'tcx , & 'a PthreadMutex > where 'tcx : 'a , { let mutex = ecx . deref_pointer_as (mutex_ptr , ecx . libc_ty_layout ("pthread_mutex_t")) ? ; ecx . lazy_sync_get_data (& mutex , mutex_init_offset (ecx) ? , | | throw_ub_format ! ("`pthread_mutex_t` can't be moved after first use") , | ecx | { let kind = mutex_kind_from_static_initializer (ecx , & mutex) ? ; interp_ok (PthreadMutex { mutex_ref : MutexRef :: new () , kind }) } ,) }
};
}
