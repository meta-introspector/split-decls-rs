// Generated macro for get_vtable (function)
macro_rules! Depcrate_vtableget_vtable {
() => {
// Module: crate::vtable
// Provides: {"get_vtable"}
// Dependencies: {}
pub (crate) fn get_vtable < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , ty : Ty < 'tcx > , trait_ref : Option < ty :: ExistentialTraitRef < 'tcx > > ,) -> Value { let data_id = data_id_for_vtable (fx . tcx , & mut fx . constants_cx , fx . module , ty , trait_ref) ; let local_data_id = fx . module . declare_data_in_func (data_id , fx . bcx . func) ; if fx . clif_comments . enabled () { fx . add_comment (local_data_id , "vtable") ; } fx . bcx . ins () . global_value (fx . pointer_type , local_data_id) }
};
}
