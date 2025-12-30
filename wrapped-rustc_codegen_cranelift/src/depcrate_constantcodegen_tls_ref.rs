// Generated macro for codegen_tls_ref (function)
macro_rules! Depcrate_constantcodegen_tls_ref {
() => {
// Module: crate::constant
// Provides: {"codegen_tls_ref"}
// Dependencies: {}
pub (crate) fn codegen_tls_ref < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , def_id : DefId , layout : TyAndLayout < 'tcx > ,) -> CValue < 'tcx > { let tls_ptr = if ! def_id . is_local () && fx . tcx . needs_thread_local_shim (def_id) { let instance = ty :: Instance { def : ty :: InstanceKind :: ThreadLocalShim (def_id) , args : ty :: GenericArgs :: empty () , } ; let func_ref = fx . get_function_ref (instance) ; let call = fx . bcx . ins () . call (func_ref , & []) ; fx . bcx . func . dfg . first_result (call) } else { let data_id = data_id_for_static (fx . tcx , fx . module , def_id , false , false ,) ; let local_data_id = fx . module . declare_data_in_func (data_id , & mut fx . bcx . func) ; if fx . clif_comments . enabled () { fx . add_comment (local_data_id , format ! ("tls {:?}" , def_id)) ; } fx . bcx . ins () . tls_value (fx . pointer_type , local_data_id) } ; CValue :: by_val (tls_ptr , layout) }
};
}
