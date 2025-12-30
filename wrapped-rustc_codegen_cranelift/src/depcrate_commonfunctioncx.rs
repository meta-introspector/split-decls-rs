// Generated macro for FunctionCx (struct)
macro_rules! Depcrate_commonFunctionCx {
() => {
// Module: crate::common
// Provides: {"FunctionCx"}
// Dependencies: {}
pub (crate) struct FunctionCx < 'm , 'clif , 'tcx : 'm > { pub (crate) cx : & 'clif mut crate :: CodegenCx , pub (crate) module : & 'm mut dyn Module , pub (crate) tcx : TyCtxt < 'tcx > , pub (crate) target_config : TargetFrontendConfig , pub (crate) pointer_type : Type , pub (crate) constants_cx : ConstantCx , pub (crate) func_debug_cx : Option < FunctionDebugContext > , pub (crate) instance : Instance < 'tcx > , pub (crate) symbol_name : String , pub (crate) mir : & 'tcx Body < 'tcx > , pub (crate) fn_abi : & 'tcx FnAbi < 'tcx , Ty < 'tcx > > , pub (crate) bcx : FunctionBuilder < 'clif > , pub (crate) block_map : IndexVec < BasicBlock , Block > , pub (crate) local_map : IndexVec < Local , CPlace < 'tcx > > , # [doc = " When `#[track_caller]` is used, the implicit caller location is stored in this variable."] pub (crate) caller_location : Option < CValue < 'tcx > > , pub (crate) clif_comments : crate :: pretty_clif :: CommentWriter , # [doc = " This should only be accessed by `CPlace::new_var`."] pub (crate) next_ssa_var : u32 , }
};
}
