// Generated macro for cond_create (function)
macro_rules! Depcrate_shims_unix_synccond_create {
() => {
// Module: crate::shims::unix::sync
// Provides: {"cond_create"}
// Dependencies: {}
fn cond_create < 'tcx > (ecx : & mut MiriInterpCx < 'tcx > , cond_ptr : & OpTy < 'tcx > , clock : TimeoutClock ,) -> InterpResult < 'tcx , PthreadCondvar > { let cond = ecx . deref_pointer_as (cond_ptr , ecx . libc_ty_layout ("pthread_cond_t")) ? ; let data = PthreadCondvar { condvar_ref : CondvarRef :: new () , clock } ; ecx . lazy_sync_init (& cond , cond_init_offset (ecx) ? , data . clone ()) ? ; interp_ok (data) }
};
}
