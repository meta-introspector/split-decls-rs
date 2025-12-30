// Generated macro for bytewise_equal (function)
macro_rules! Depcrate_shims_unix_syncbytewise_equal {
() => {
// Module: crate::shims::unix::sync
// Provides: {"bytewise_equal"}
// Dependencies: {}
# [doc = " Do a bytewise comparison of the two places. This is used to check if"] # [doc = " a synchronization primitive matches its static initializer value."] fn bytewise_equal < 'tcx > (ecx : & MiriInterpCx < 'tcx > , left : & MPlaceTy < 'tcx > , right : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , bool > { let size = left . layout . size ; assert_eq ! (size , right . layout . size) ; let left_bytes = ecx . read_bytes_ptr_strip_provenance (left . ptr () , size) ? ; let right_bytes = ecx . read_bytes_ptr_strip_provenance (right . ptr () , size) ? ; interp_ok (left_bytes == right_bytes) }
};
}
