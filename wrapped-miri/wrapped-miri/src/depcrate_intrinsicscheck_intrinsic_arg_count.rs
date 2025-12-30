// Generated macro for check_intrinsic_arg_count (function)
macro_rules! Depcrate_intrinsicscheck_intrinsic_arg_count {
() => {
// Module: crate::intrinsics
// Provides: {"check_intrinsic_arg_count"}
// Dependencies: {}
# [doc = " Check that the number of args is what we expect."] fn check_intrinsic_arg_count < 'a , 'tcx , const N : usize > (args : & 'a [OpTy < 'tcx >] ,) -> InterpResult < 'tcx , & 'a [OpTy < 'tcx > ; N] > where & 'a [OpTy < 'tcx > ; N] : TryFrom < & 'a [OpTy < 'tcx >] > , { if let Ok (ops) = args . try_into () { return interp_ok (ops) ; } throw_ub_format ! ("incorrect number of arguments for intrinsic: got {}, expected {}" , args . len () , N) }
};
}
