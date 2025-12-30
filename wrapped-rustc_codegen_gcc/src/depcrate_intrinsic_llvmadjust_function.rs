// Generated macro for adjust_function (function)
macro_rules! Depcrate_intrinsic_llvmadjust_function {
() => {
// Module: crate::intrinsic::llvm
// Provides: {"adjust_function"}
// Dependencies: {}
# [cfg_attr (not (feature = "master") , allow (unused_variables))] pub fn adjust_function < 'gcc > (context : & 'gcc Context < 'gcc > , func_name : & str , func_ptr : RValue < 'gcc > , args : & [RValue < 'gcc >] ,) -> RValue < 'gcc > { # [cfg (feature = "master")] match func_name { "__builtin_ia32_vfcmaddcsh_mask3_round" => { if format ! ("{:?}" , args [3]) . ends_with ("255") { return context . get_target_builtin_function ("__builtin_ia32_vfcmaddcsh_mask_round") . get_address (None) ; } } "__builtin_ia32_vfmaddcsh_mask3_round" => { if format ! ("{:?}" , args [3]) . ends_with ("255") { return context . get_target_builtin_function ("__builtin_ia32_vfmaddcsh_mask_round") . get_address (None) ; } } _ => () , } func_ptr }
};
}
