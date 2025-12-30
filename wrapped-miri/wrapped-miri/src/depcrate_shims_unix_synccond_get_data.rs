// Generated macro for cond_get_data (function)
macro_rules! Depcrate_shims_unix_synccond_get_data {
() => {
// Module: crate::shims::unix::sync
// Provides: {"cond_get_data"}
// Dependencies: {}
fn cond_get_data < 'tcx , 'a > (ecx : & 'a mut MiriInterpCx < 'tcx > , cond_ptr : & OpTy < 'tcx > ,) -> InterpResult < 'tcx , & 'a PthreadCondvar > where 'tcx : 'a , { let cond = ecx . deref_pointer_as (cond_ptr , ecx . libc_ty_layout ("pthread_cond_t")) ? ; ecx . lazy_sync_get_data (& cond , cond_init_offset (ecx) ? , | | throw_ub_format ! ("`pthread_cond_t` can't be moved after first use") , | ecx | { if ! bytewise_equal_atomic_relaxed (ecx , & cond , & ecx . eval_path (& ["libc" , "PTHREAD_COND_INITIALIZER"]) ,) ? { throw_unsup_format ! ("unsupported static initializer used for `pthread_cond_t`") ; } interp_ok (PthreadCondvar { condvar_ref : CondvarRef :: new () , clock : TimeoutClock :: RealTime , }) } ,) }
};
}
