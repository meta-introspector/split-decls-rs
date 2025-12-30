// Generated macro for ignore_arg_cast (function)
macro_rules! Depcrate_intrinsic_llvmignore_arg_cast {
() => {
// Module: crate::intrinsic::llvm
// Provides: {"ignore_arg_cast"}
// Dependencies: {}
pub fn ignore_arg_cast (func_name : & str , index : usize , args_len : usize) -> bool { match func_name { "__builtin_ia32_maxps512_mask" | "__builtin_ia32_maxpd512_mask" | "__builtin_ia32_minps512_mask" | "__builtin_ia32_minpd512_mask" | "__builtin_ia32_sqrtps512_mask" | "__builtin_ia32_sqrtpd512_mask" | "__builtin_ia32_addps512_mask" | "__builtin_ia32_addpd512_mask" | "__builtin_ia32_subps512_mask" | "__builtin_ia32_subpd512_mask" | "__builtin_ia32_mulps512_mask" | "__builtin_ia32_mulpd512_mask" | "__builtin_ia32_divps512_mask" | "__builtin_ia32_divpd512_mask" | "__builtin_ia32_vfmaddsubps512_mask" | "__builtin_ia32_vfmaddsubpd512_mask" | "__builtin_ia32_cvtdq2ps512_mask" | "__builtin_ia32_cvtudq2ps512_mask" => { if index == args_len - 1 { return true ; } } "__builtin_ia32_rndscaless_mask_round" | "__builtin_ia32_rndscalesd_mask_round" => { if index == 2 || index == 3 { return true ; } } "__builtin_ia32_vfmaddps512_mask" | "__builtin_ia32_vfmaddpd512_mask" => { if args_len == 4 && index == args_len - 1 { return true ; } } "__builtin_ia32_vfmaddss3_round" | "__builtin_ia32_vfmaddsd3_round" => return true , "__builtin_ia32_vplzcntd_512_mask" | "__builtin_ia32_vplzcntd_256_mask" | "__builtin_ia32_vplzcntd_128_mask" | "__builtin_ia32_vplzcntq_512_mask" | "__builtin_ia32_vplzcntq_256_mask" | "__builtin_ia32_vplzcntq_128_mask" => { if index == args_len - 1 { return true ; } } _ => () , } false }
};
}
