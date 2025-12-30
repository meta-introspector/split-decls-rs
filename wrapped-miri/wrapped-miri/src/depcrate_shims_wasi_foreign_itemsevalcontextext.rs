// Generated macro for EvalContextExt (trait)
macro_rules! Depcrate_shims_wasi_foreign_itemsEvalContextExt {
() => {
// Module: crate::shims::wasi::foreign_items
// Provides: {"EvalContextExt"}
// Dependencies: {}
pub trait EvalContextExt < 'tcx > : crate :: MiriInterpCxExt < 'tcx > { fn emulate_foreign_item_inner (& mut self , link_name : Symbol , abi : & FnAbi < 'tcx , Ty < 'tcx > > , args : & [OpTy < 'tcx >] , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , EmulateItemResult > { let this = self . eval_context_mut () ; match link_name . as_str () { "posix_memalign" => { let [memptr , align , size] = this . check_shim_sig_lenient (abi , CanonAbi :: C , link_name , args) ? ; let result = this . posix_memalign (memptr , align , size) ? ; this . write_scalar (result , dest) ? ; } "aligned_alloc" => { let [align , size] = this . check_shim_sig_lenient (abi , CanonAbi :: C , link_name , args) ? ; let res = this . aligned_alloc (align , size) ? ; this . write_pointer (res , dest) ? ; } _ => return interp_ok (EmulateItemResult :: NotSupported) , } interp_ok (EmulateItemResult :: NeedsReturn) } }
};
}
