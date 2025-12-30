// Generated macro for prctl (function)
macro_rules! Depcrate_shims_unix_android_threadprctl {
() => {
// Module: crate::shims::unix::android::thread
// Provides: {"prctl"}
// Dependencies: {}
pub fn prctl < 'tcx > (ecx : & mut MiriInterpCx < 'tcx > , link_name : Symbol , abi : & FnAbi < 'tcx , Ty < 'tcx > > , args : & [OpTy < 'tcx >] , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx > { let ([op] , varargs) = ecx . check_shim_sig_variadic_lenient (abi , CanonAbi :: C , link_name , args) ? ; let pr_set_name = 15 ; let pr_get_name = 16 ; let res = match ecx . read_scalar (op) ? . to_i32 () ? { op if op == pr_set_name => { let [name] = check_min_vararg_count ("prctl(PR_SET_NAME, ...)" , varargs) ? ; let name = ecx . read_scalar (name) ? ; let thread = ecx . pthread_self () ? ; let res = ecx . pthread_setname_np (thread , name , TASK_COMM_LEN , true) ? ; assert_eq ! (res , ThreadNameResult :: Ok) ; Scalar :: from_u32 (0) } op if op == pr_get_name => { let [name] = check_min_vararg_count ("prctl(PR_GET_NAME, ...)" , varargs) ? ; let name = ecx . read_scalar (name) ? ; let thread = ecx . pthread_self () ? ; let len = Scalar :: from_target_usize (TASK_COMM_LEN , ecx) ; ecx . check_ptr_access (name . to_pointer (ecx) ? , Size :: from_bytes (TASK_COMM_LEN) , CheckInAllocMsg :: MemoryAccess ,) ? ; let res = ecx . pthread_getname_np (thread , name , len , false) ? ; assert_eq ! (res , ThreadNameResult :: Ok) ; Scalar :: from_u32 (0) } op => throw_unsup_format ! ("Miri does not support `prctl` syscall with op={}" , op) , } ; ecx . write_scalar (res , dest) ? ; interp_ok (()) }
};
}
