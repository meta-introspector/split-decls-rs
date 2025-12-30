// Generated macro for condattr_set_clock_id (function)
macro_rules! Depcrate_shims_unix_synccondattr_set_clock_id {
() => {
// Module: crate::shims::unix::sync
// Provides: {"condattr_set_clock_id"}
// Dependencies: {}
fn condattr_set_clock_id < 'tcx > (ecx : & mut MiriInterpCx < 'tcx > , attr_ptr : & OpTy < 'tcx > , clock_id : i32 ,) -> InterpResult < 'tcx , () > { ecx . deref_pointer_and_write (attr_ptr , condattr_clock_offset (ecx) ? , Scalar :: from_i32 (clock_id) , ecx . libc_ty_layout ("pthread_condattr_t") , ecx . machine . layouts . i32 ,) }
};
}
