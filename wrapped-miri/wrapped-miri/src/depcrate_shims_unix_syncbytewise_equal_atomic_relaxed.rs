// Generated macro for bytewise_equal_atomic_relaxed (function)
macro_rules! Depcrate_shims_unix_syncbytewise_equal_atomic_relaxed {
() => {
// Module: crate::shims::unix::sync
// Provides: {"bytewise_equal_atomic_relaxed"}
// Dependencies: {}
# [doc = " Do a bytewise comparison of the two places, using relaxed atomic reads. This is used to check if"] # [doc = " a synchronization primitive matches its static initializer value."] # [doc = ""] # [doc = " The reads happen in chunks of 4, so all racing accesses must also use that access size."] fn bytewise_equal_atomic_relaxed < 'tcx > (ecx : & MiriInterpCx < 'tcx > , left : & MPlaceTy < 'tcx > , right : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , bool > { let size = left . layout . size ; assert_eq ! (size , right . layout . size) ; assert ! (size . bytes () . is_multiple_of (4)) ; for i in 0 .. (size . bytes () / 4) { let offset = Size :: from_bytes (i . strict_mul (4)) ; let load = | place : & MPlaceTy < 'tcx > | { let byte = place . offset (offset , ecx . machine . layouts . u32 , ecx) ? ; ecx . read_scalar_atomic (& byte , AtomicReadOrd :: Relaxed) ? . to_u32 () } ; let left = load (left) ? ; let right = load (right) ? ; if left != right { return interp_ok (false) ; } } interp_ok (true) }
};
}
