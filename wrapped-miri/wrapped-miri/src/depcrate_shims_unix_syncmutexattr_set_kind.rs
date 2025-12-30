// Generated macro for mutexattr_set_kind (function)
macro_rules! Depcrate_shims_unix_syncmutexattr_set_kind {
() => {
// Module: crate::shims::unix::sync
// Provides: {"mutexattr_set_kind"}
// Dependencies: {}
fn mutexattr_set_kind < 'tcx > (ecx : & mut MiriInterpCx < 'tcx > , attr_ptr : & OpTy < 'tcx > , kind : i32 ,) -> InterpResult < 'tcx , () > { ecx . deref_pointer_and_write (attr_ptr , mutexattr_kind_offset (ecx) ? , Scalar :: from_i32 (kind) , ecx . libc_ty_layout ("pthread_mutexattr_t") , ecx . machine . layouts . i32 ,) }
};
}
