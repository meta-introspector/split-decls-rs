// Generated macro for alloc_type_name (function)
macro_rules! Depcrate_interpret_intrinsicsalloc_type_name {
() => {
// Module: crate::interpret::intrinsics
// Provides: {"alloc_type_name"}
// Dependencies: {}
# [doc = " Directly returns an `Allocation` containing an absolute path representation of the given type."] pub (crate) fn alloc_type_name < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> (AllocId , u64) { let path = crate :: util :: type_name (tcx , ty) ; let bytes = path . into_bytes () ; let len = bytes . len () . try_into () . unwrap () ; (tcx . allocate_bytes_dedup (bytes , CTFE_ALLOC_SALT) , len) }
};
}
