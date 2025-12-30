// Generated macro for rwlock_get_data (function)
macro_rules! Depcrate_shims_unix_syncrwlock_get_data {
() => {
// Module: crate::shims::unix::sync
// Provides: {"rwlock_get_data"}
// Dependencies: {}
fn rwlock_get_data < 'tcx , 'a > (ecx : & 'a mut MiriInterpCx < 'tcx > , rwlock_ptr : & OpTy < 'tcx > ,) -> InterpResult < 'tcx , & 'a PthreadRwLock > where 'tcx : 'a , { let rwlock = ecx . deref_pointer_as (rwlock_ptr , ecx . libc_ty_layout ("pthread_rwlock_t")) ? ; ecx . lazy_sync_get_data (& rwlock , rwlock_init_offset (ecx) ? , | | throw_ub_format ! ("`pthread_rwlock_t` can't be moved after first use") , | ecx | { if ! bytewise_equal_atomic_relaxed (ecx , & rwlock , & ecx . eval_path (& ["libc" , "PTHREAD_RWLOCK_INITIALIZER"]) ,) ? { throw_unsup_format ! ("unsupported static initializer used for `pthread_rwlock_t`") ; } interp_ok (PthreadRwLock { rwlock_ref : RwLockRef :: new () }) } ,) }
};
}
