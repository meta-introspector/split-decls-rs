// Generated macro for build_fixed_size_array_di_node (function)
macro_rules! Depcrate_debuginfo_metadatabuild_fixed_size_array_di_node {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"build_fixed_size_array_di_node"}
// Dependencies: {}
# [doc = " Creates debuginfo for a fixed size array (e.g. `[u64; 123]`)."] # [doc = " For slices (that is, \"arrays\" of unknown size) use [build_slice_type_di_node]."] fn build_fixed_size_array_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > , array_type : Ty < 'tcx > , span : Span ,) -> DINodeCreationResult < 'll > { let ty :: Array (element_type , len) = array_type . kind () else { bug ! ("build_fixed_size_array_di_node() called with non-ty::Array type `{:?}`" , array_type) } ; let element_type_di_node = spanned_type_di_node (cx , * element_type , span) ; return_if_di_node_created_in_meantime ! (cx , unique_type_id) ; let (size , align) = cx . spanned_size_and_align_of (array_type , span) ; let upper_bound = len . try_to_target_usize (cx . tcx) . expect ("expected monomorphic const in codegen") as c_longlong ; let subrange = unsafe { Some (llvm :: LLVMRustDIBuilderGetOrCreateSubrange (DIB (cx) , 0 , upper_bound)) } ; let subscripts = create_DIArray (DIB (cx) , & [subrange]) ; let di_node = unsafe { llvm :: LLVMRustDIBuilderCreateArrayType (DIB (cx) , size . bits () , align . bits () as u32 , element_type_di_node , subscripts ,) } ; DINodeCreationResult :: new (di_node , false) }
};
}
