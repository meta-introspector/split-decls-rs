// Generated macro for pointer_for_allocation (function)
macro_rules! Depcrate_constantpointer_for_allocation {
() => {
// Module: crate::constant
// Provides: {"pointer_for_allocation"}
// Dependencies: {}
fn pointer_for_allocation < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , alloc_id : AllocId ,) -> crate :: pointer :: Pointer { let alloc = fx . tcx . global_alloc (alloc_id) . unwrap_memory () ; let data_id = data_id_for_alloc_id (& mut fx . constants_cx , fx . module , alloc_id , alloc . inner () . mutability) ; let local_data_id = fx . module . declare_data_in_func (data_id , & mut fx . bcx . func) ; if fx . clif_comments . enabled () { fx . add_comment (local_data_id , format ! ("{:?}" , alloc_id)) ; } let global_ptr = fx . bcx . ins () . global_value (fx . pointer_type , local_data_id) ; crate :: pointer :: Pointer :: new (global_ptr) }
};
}
