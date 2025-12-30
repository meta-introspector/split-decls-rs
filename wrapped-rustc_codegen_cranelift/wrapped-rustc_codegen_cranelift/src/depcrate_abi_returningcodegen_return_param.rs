// Generated macro for codegen_return_param (function)
macro_rules! Depcrate_abi_returningcodegen_return_param {
() => {
// Module: crate::abi::returning
// Provides: {"codegen_return_param"}
// Dependencies: {}
# [doc = " Return a place where the return value of the current function can be written to. If necessary"] # [doc = " this adds an extra parameter pointing to where the return value needs to be stored."] pub (super) fn codegen_return_param < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , ssa_analyzed : & rustc_index :: IndexSlice < Local , crate :: analyze :: SsaKind > , block_params_iter : & mut impl Iterator < Item = Value > ,) -> CPlace < 'tcx > { let (ret_place , ret_param) : (_ , SmallVec < [_ ; 2] >) = match fx . fn_abi . ret . mode { PassMode :: Ignore | PassMode :: Direct (_) | PassMode :: Pair (_ , _) | PassMode :: Cast { .. } => { let is_ssa = ssa_analyzed [RETURN_PLACE] . is_ssa (fx , fx . fn_abi . ret . layout . ty) ; (super :: make_local_place (fx , RETURN_PLACE , fx . fn_abi . ret . layout , is_ssa) , smallvec ! []) } PassMode :: Indirect { attrs : _ , meta_attrs : None , on_stack : _ } => { let ret_param = block_params_iter . next () . unwrap () ; assert_eq ! (fx . bcx . func . dfg . value_type (ret_param) , fx . pointer_type) ; (CPlace :: for_ptr (Pointer :: new (ret_param) , fx . fn_abi . ret . layout) , smallvec ! [ret_param]) } PassMode :: Indirect { attrs : _ , meta_attrs : Some (_) , on_stack : _ } => { unreachable ! ("unsized return value") } } ; crate :: abi :: comments :: add_arg_comment (fx , "ret" , Some (RETURN_PLACE) , None , & ret_param , & fx . fn_abi . ret . mode , fx . fn_abi . ret . layout ,) ; ret_place }
};
}
