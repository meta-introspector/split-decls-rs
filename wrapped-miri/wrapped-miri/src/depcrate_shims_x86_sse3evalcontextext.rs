// Generated macro for EvalContextExt (trait)
macro_rules! Depcrate_shims_x86_sse3EvalContextExt {
() => {
// Module: crate::shims::x86::sse3
// Provides: {"EvalContextExt"}
// Dependencies: {}
pub (super) trait EvalContextExt < 'tcx > : crate :: MiriInterpCxExt < 'tcx > { fn emulate_x86_sse3_intrinsic (& mut self , link_name : Symbol , abi : & FnAbi < 'tcx , Ty < 'tcx > > , args : & [OpTy < 'tcx >] , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , EmulateItemResult > { let this = self . eval_context_mut () ; this . expect_target_feature_for_intrinsic (link_name , "sse3") ? ; let unprefixed_name = link_name . as_str () . strip_prefix ("llvm.x86.sse3.") . unwrap () ; match unprefixed_name { "hadd.ps" | "hadd.pd" | "hsub.ps" | "hsub.pd" => { let [left , right] = this . check_shim_sig_lenient (abi , CanonAbi :: C , link_name , args) ? ; let which = match unprefixed_name { "hadd.ps" | "hadd.pd" => mir :: BinOp :: Add , "hsub.ps" | "hsub.pd" => mir :: BinOp :: Sub , _ => unreachable ! () , } ; horizontal_bin_op (this , which , false , left , right , dest) ? ; } "ldu.dq" => { let [src_ptr] = this . check_shim_sig_lenient (abi , CanonAbi :: C , link_name , args) ? ; let src_ptr = this . read_pointer (src_ptr) ? ; let dest = dest . force_mplace (this) ? ; this . mem_copy (src_ptr , dest . ptr () , dest . layout . size , true) ? ; } _ => return interp_ok (EmulateItemResult :: NotSupported) , } interp_ok (EmulateItemResult :: NeedsReturn) } }
};
}
