// Generated macro for adjust_arg_for_abi (function)
macro_rules! Depcrate_abi_pass_modeadjust_arg_for_abi {
() => {
// Module: crate::abi::pass_mode
// Provides: {"adjust_arg_for_abi"}
// Dependencies: {}
# [doc = " Get a set of values to be passed as function arguments."] pub (super) fn adjust_arg_for_abi < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , arg : CValue < 'tcx > , arg_abi : & ArgAbi < 'tcx , Ty < 'tcx > > , is_owned : bool ,) -> SmallVec < [Value ; 2] > { assert_assignable (fx , arg . layout () . ty , arg_abi . layout . ty , 16) ; match arg_abi . mode { PassMode :: Ignore => smallvec ! [] , PassMode :: Direct (_) => smallvec ! [arg . load_scalar (fx)] , PassMode :: Pair (_ , _) => { let (a , b) = arg . load_scalar_pair (fx) ; smallvec ! [a , b] } PassMode :: Cast { ref cast , .. } => to_casted_value (fx , arg , cast) , PassMode :: Indirect { .. } => { if is_owned { match arg . force_stack (fx) { (ptr , None) => smallvec ! [ptr . get_addr (fx)] , (ptr , Some (meta)) => smallvec ! [ptr . get_addr (fx) , meta] , } } else { let place = CPlace :: new_stack_slot (fx , arg . layout ()) ; place . write_cvalue (fx , arg) ; smallvec ! [place . to_ptr () . get_addr (fx)] } } } }
};
}
