// Generated macro for mutexattr_get_kind (function)
macro_rules! Depcrate_shims_unix_syncmutexattr_get_kind {
() => {
// Module: crate::shims::unix::sync
// Provides: {"mutexattr_get_kind"}
// Dependencies: {}
fn mutexattr_get_kind < 'tcx > (ecx : & MiriInterpCx < 'tcx > , attr_ptr : & OpTy < 'tcx > ,) -> InterpResult < 'tcx , i32 > { ecx . deref_pointer_and_read (attr_ptr , mutexattr_kind_offset (ecx) ? , ecx . libc_ty_layout ("pthread_mutexattr_t") , ecx . machine . layouts . i32 ,) ? . to_i32 () }
};
}
