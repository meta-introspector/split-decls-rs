// Generated macro for check_min_vararg_count (function)
macro_rules! Depcrate_shims_sigcheck_min_vararg_count {
() => {
// Module: crate::shims::sig
// Provides: {"check_min_vararg_count"}
// Dependencies: {}
# [doc = " Check that the number of varargs is at least the minimum what we expect."] # [doc = " Fixed args should not be included."] pub fn check_min_vararg_count < 'a , 'tcx , const N : usize > (name : & 'a str , args : & 'a [OpTy < 'tcx >] ,) -> InterpResult < 'tcx , & 'a [OpTy < 'tcx > ; N] > { if let Some ((ops , _)) = args . split_first_chunk () { return interp_ok (ops) ; } throw_ub_format ! ("not enough variadic arguments for `{name}`: got {}, expected at least {}" , args . len () , N) }
};
}
