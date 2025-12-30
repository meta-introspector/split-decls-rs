// Generated macro for condattr_get_clock_id (function)
macro_rules! Depcrate_shims_unix_synccondattr_get_clock_id {
() => {
// Module: crate::shims::unix::sync
// Provides: {"condattr_get_clock_id"}
// Dependencies: {}
fn condattr_get_clock_id < 'tcx > (ecx : & MiriInterpCx < 'tcx > , attr_ptr : & OpTy < 'tcx > ,) -> InterpResult < 'tcx , Scalar > { ecx . deref_pointer_and_read (attr_ptr , condattr_clock_offset (ecx) ? , ecx . libc_ty_layout ("pthread_condattr_t") , ecx . machine . layouts . i32 ,) }
};
}
