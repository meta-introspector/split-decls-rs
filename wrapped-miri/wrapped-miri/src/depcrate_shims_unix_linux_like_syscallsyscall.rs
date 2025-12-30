// Generated macro for syscall (function)
macro_rules! Depcrate_shims_unix_linux_like_syscallsyscall {
() => {
// Module: crate::shims::unix::linux_like::syscall
// Provides: {"syscall"}
// Dependencies: {}
pub fn syscall < 'tcx > (ecx : & mut MiriInterpCx < 'tcx > , link_name : Symbol , abi : & FnAbi < 'tcx , Ty < 'tcx > > , args : & [OpTy < 'tcx >] , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx > { let ([op] , varargs) = ecx . check_shim_sig_variadic_lenient (abi , CanonAbi :: C , link_name , args) ? ; let sys_getrandom = ecx . eval_libc ("SYS_getrandom") . to_target_usize (ecx) ? ; let sys_futex = ecx . eval_libc ("SYS_futex") . to_target_usize (ecx) ? ; let sys_eventfd2 = ecx . eval_libc ("SYS_eventfd2") . to_target_usize (ecx) ? ; let sys_gettid = ecx . eval_libc ("SYS_gettid") . to_target_usize (ecx) ? ; match ecx . read_target_usize (op) ? { num if num == sys_getrandom => { let [ptr , len , flags] = check_min_vararg_count ("syscall(SYS_getrandom, ...)" , varargs) ? ; let ptr = ecx . read_pointer (ptr) ? ; let len = ecx . read_target_usize (len) ? ; let _flags = ecx . read_scalar (flags) ? . to_i32 () ? ; ecx . gen_random (ptr , len) ? ; ecx . write_scalar (Scalar :: from_target_usize (len , ecx) , dest) ? ; } num if num == sys_futex => { futex (ecx , varargs , dest) ? ; } num if num == sys_eventfd2 => { let [initval , flags] = check_min_vararg_count ("syscall(SYS_evetfd2, ...)" , varargs) ? ; let result = ecx . eventfd (initval , flags) ? ; ecx . write_int (result . to_i32 () ? , dest) ? ; } num if num == sys_gettid => { let result = ecx . unix_gettid ("SYS_gettid") ? ; ecx . write_int (result . to_u32 () ? , dest) ? ; } num => { throw_unsup_format ! ("syscall: unsupported syscall number {num}") ; } } ; interp_ok (()) }
};
}
