// Generated macro for codegen_return (function)
macro_rules! Depcrate_abi_returningcodegen_return {
() => {
// Module: crate::abi::returning
// Provides: {"codegen_return"}
// Dependencies: {}
# [doc = " Codegen a return instruction with the right return value(s) if any."] pub (crate) fn codegen_return (fx : & mut FunctionCx < '_ , '_ , '_ >) { match fx . fn_abi . ret . mode { PassMode :: Ignore | PassMode :: Indirect { attrs : _ , meta_attrs : None , on_stack : _ } => { fx . bcx . ins () . return_ (& []) ; } PassMode :: Indirect { attrs : _ , meta_attrs : Some (_) , on_stack : _ } => { unreachable ! ("unsized return value") } PassMode :: Direct (_) => { let place = fx . get_local_place (RETURN_PLACE) ; let ret_val = place . to_cvalue (fx) . load_scalar (fx) ; fx . bcx . ins () . return_ (& [ret_val]) ; } PassMode :: Pair (_ , _) => { let place = fx . get_local_place (RETURN_PLACE) ; let (ret_val_a , ret_val_b) = place . to_cvalue (fx) . load_scalar_pair (fx) ; fx . bcx . ins () . return_ (& [ret_val_a , ret_val_b]) ; } PassMode :: Cast { ref cast , .. } => { let place = fx . get_local_place (RETURN_PLACE) ; let ret_val = place . to_cvalue (fx) ; let ret_vals = super :: pass_mode :: to_casted_value (fx , ret_val , cast) ; fx . bcx . ins () . return_ (& ret_vals) ; } } }
};
}
