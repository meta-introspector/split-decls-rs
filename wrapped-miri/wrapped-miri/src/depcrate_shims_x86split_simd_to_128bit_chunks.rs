// Generated macro for split_simd_to_128bit_chunks (function)
macro_rules! Depcrate_shims_x86split_simd_to_128bit_chunks {
() => {
// Module: crate::shims::x86
// Provides: {"split_simd_to_128bit_chunks"}
// Dependencies: {}
# [doc = " Splits `op` (which must be a SIMD vector) into 128-bit chunks."] # [doc = ""] # [doc = " Returns a tuple where:"] # [doc = " * The first element is the number of 128-bit chunks (let's call it `N`)."] # [doc = " * The second element is the number of elements per chunk (let's call it `M`)."] # [doc = " * The third element is the `op` vector split into chunks, i.e, it's"] # [doc = "   type is `[[T; M]; N]` where `T` is the element type of `op`."] fn split_simd_to_128bit_chunks < 'tcx , P : Projectable < 'tcx , Provenance > > (ecx : & mut crate :: MiriInterpCx < 'tcx > , op : & P ,) -> InterpResult < 'tcx , (u64 , u64 , P) > { let simd_layout = op . layout () ; let (simd_len , element_ty) = simd_layout . ty . simd_size_and_type (ecx . tcx . tcx) ; assert_eq ! (simd_layout . size . bits () % 128 , 0) ; let num_chunks = simd_layout . size . bits () / 128 ; let items_per_chunk = simd_len . strict_div (num_chunks) ; let chunked_layout = ecx . layout_of (Ty :: new_array (ecx . tcx . tcx , Ty :: new_array (ecx . tcx . tcx , element_ty , items_per_chunk) , num_chunks ,)) . unwrap () ; let chunked_op = op . transmute (chunked_layout , ecx) ? ; interp_ok ((num_chunks , items_per_chunk , chunked_op)) }
};
}
