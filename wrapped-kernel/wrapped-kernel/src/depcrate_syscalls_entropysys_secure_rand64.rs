// Generated macro for sys_secure_rand64 (function)
macro_rules! Depcrate_syscalls_entropysys_secure_rand64 {
() => {
// Module: crate::syscalls::entropy
// Provides: {"sys_secure_rand64"}
// Dependencies: {}
# [doc = " Create a cryptographicly secure 64bit random number with the support of"] # [doc = " the underlying hardware. If the required hardware isn't available,"] # [doc = " the function returns -1."] # [cfg (not (feature = "newlib"))] # [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_secure_rand64 (value : * mut u64) -> i32 { let mut buf = value . cast () ; let mut len = size_of :: < u64 > () ; while len != 0 { let res = unsafe { read_entropy (buf , len , 0) } ; if res < 0 { return - 1 ; } buf = unsafe { buf . add (res as usize) } ; len -= res as usize ; } 0 }
};
}
