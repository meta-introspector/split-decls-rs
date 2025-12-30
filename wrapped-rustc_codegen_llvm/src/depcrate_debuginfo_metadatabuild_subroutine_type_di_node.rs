// Generated macro for build_subroutine_type_di_node (function)
macro_rules! Depcrate_debuginfo_metadatabuild_subroutine_type_di_node {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"build_subroutine_type_di_node"}
// Dependencies: {}
fn build_subroutine_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { debug_context (cx) . type_map . unique_id_to_di_node . borrow_mut () . insert (unique_type_id , recursion_marker_type_di_node (cx)) ; let fn_ty = unique_type_id . expect_ty () ; let signature = cx . tcx . normalize_erasing_late_bound_regions (cx . typing_env () , fn_ty . fn_sig (cx . tcx)) ; let signature_di_nodes : SmallVec < _ > = iter :: once (match signature . output () . kind () { ty :: Tuple (tys) if tys . is_empty () => { None } _ => Some (type_di_node (cx , signature . output ())) , } ,) . chain (signature . inputs () . iter () . map (| & argument_type | Some (type_di_node (cx , argument_type))) ,) . collect () ; debug_context (cx) . type_map . unique_id_to_di_node . borrow_mut () . remove (& unique_type_id) ; let fn_di_node = create_subroutine_type (cx , create_DIArray (DIB (cx) , & signature_di_nodes [..])) ; let name = compute_debuginfo_type_name (cx . tcx , fn_ty , false) ; let (size , align) = match fn_ty . kind () { ty :: FnDef (..) => (Size :: ZERO , Align :: ONE) , ty :: FnPtr (..) => { (cx . tcx . data_layout . pointer_size () , cx . tcx . data_layout . pointer_align () . abi) } _ => unreachable ! () , } ; let di_node = unsafe { llvm :: LLVMRustDIBuilderCreatePointerType (DIB (cx) , fn_di_node , size . bits () , align . bits () as u32 , 0 , name . as_c_char_ptr () , name . len () ,) } ; DINodeCreationResult :: new (di_node , false) }
};
}
