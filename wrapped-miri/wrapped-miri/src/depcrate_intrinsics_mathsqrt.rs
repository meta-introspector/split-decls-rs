// Generated macro for sqrt (function)
macro_rules! Depcrate_intrinsics_mathsqrt {
() => {
// Module: crate::intrinsics::math
// Provides: {"sqrt"}
// Dependencies: {}
fn sqrt < 'tcx , F : Float + FloatConvert < F > + Into < Scalar > > (this : & mut MiriInterpCx < 'tcx > , args : & [OpTy < 'tcx >] , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx > { let [f] = check_intrinsic_arg_count (args) ? ; let f = this . read_scalar (f) ? ; let f : F = f . to_float () ? ; let res = math :: sqrt (f) ; let res = this . adjust_nan (res , & [f]) ; this . write_scalar (res , dest) }
};
}
