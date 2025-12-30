// Generated macro for __detect_target_features (macro)
macro_rules! Depcrate_x86__detect_target_features {
() => {
// Module: crate::x86
// Provides: {"__detect_target_features"}
// Dependencies: {}
# [doc = " Use CPUID to detect the presence of all supplied target features."] # [macro_export] # [doc (hidden)] macro_rules ! __detect_target_features { ($ ($ tf : tt) ,+) => { { # [cfg (target_arch = "x86")] use core :: arch :: x86 :: { __cpuid , __cpuid_count , CpuidResult } ; # [cfg (target_arch = "x86_64")] use core :: arch :: x86_64 :: { __cpuid , __cpuid_count , CpuidResult } ; unsafe fn cpuid (leaf : u32) -> CpuidResult { __cpuid (leaf) } unsafe fn cpuid_count (leaf : u32 , sub_leaf : u32) -> CpuidResult { __cpuid_count (leaf , sub_leaf) } let cr = unsafe { [cpuid (1) , cpuid_count (7 , 0) , cpuid_count (7 , 1)] } ; $ ($ crate :: check ! (cr , $ tf) &) + true } } ; }
};
}
