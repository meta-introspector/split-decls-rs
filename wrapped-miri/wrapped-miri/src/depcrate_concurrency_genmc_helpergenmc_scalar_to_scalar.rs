// Generated macro for genmc_scalar_to_scalar (function)
macro_rules! Depcrate_concurrency_genmc_helpergenmc_scalar_to_scalar {
() => {
// Module: crate::concurrency::genmc::helper
// Provides: {"genmc_scalar_to_scalar"}
// Dependencies: {}
# [doc = " Inverse function to `scalar_to_genmc_scalar`."] # [doc = ""] # [doc = " Convert a `GenmcScalar` back into a Miri `Scalar`."] # [doc = " For pointers, attempt to convert the stored base address of their allocation back into an `AllocId`."] pub fn genmc_scalar_to_scalar < 'tcx > (ecx : & MiriInterpCx < 'tcx > , genmc_ctx : & GenmcCtx , scalar : GenmcScalar , size : Size ,) -> InterpResult < 'tcx , Scalar > { if scalar . provenance == 0 { let (value_scalar_int , _got_truncated) = ScalarInt :: truncate_from_uint (scalar . value , size) ; return interp_ok (Scalar :: from (value_scalar_int)) ; } let alloc_id = genmc_ctx . exec_state . genmc_shared_allocs_map . borrow () [& scalar . provenance] ; let provenance = machine :: Provenance :: Concrete { alloc_id , tag : BorTag :: default () } ; let ptr = interpret :: Pointer :: new (provenance , Size :: from_bytes (scalar . value)) ; interp_ok (Scalar :: from_pointer (ptr , & ecx . tcx)) }
};
}
