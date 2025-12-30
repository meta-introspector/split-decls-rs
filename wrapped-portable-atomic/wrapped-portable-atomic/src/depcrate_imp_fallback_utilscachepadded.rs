// Generated macro for CachePadded (struct)
macro_rules! Depcrate_imp_fallback_utilsCachePadded {
() => {
// Module: crate::imp::fallback::utils
// Provides: {"CachePadded"}
// Dependencies: {}
# [doc = " Pads and aligns a value to the length of a cache line."] # [cfg_attr (any (target_arch = "x86_64" , target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "powerpc64" ,) , repr (align (128)))] # [cfg_attr (any (target_arch = "arm" , target_arch = "mips" , target_arch = "mips32r6" , target_arch = "mips64" , target_arch = "mips64r6" , target_arch = "sparc" , target_arch = "hexagon" ,) , repr (align (32)))] # [cfg_attr (target_arch = "m68k" , repr (align (16)))] # [cfg_attr (target_arch = "s390x" , repr (align (256)))] # [cfg_attr (not (any (target_arch = "x86_64" , target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "powerpc64" , target_arch = "arm" , target_arch = "mips" , target_arch = "mips32r6" , target_arch = "mips64" , target_arch = "mips64r6" , target_arch = "sparc" , target_arch = "hexagon" , target_arch = "m68k" , target_arch = "s390x" ,)) , repr (align (64)))] pub (crate) struct CachePadded < T > { value : T , }
};
}
