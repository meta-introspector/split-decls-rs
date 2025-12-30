// Generated macro for sys_read_entropy (function)
macro_rules! Depcrate_syscalls_entropysys_read_entropy {
() => {
// Module: crate::syscalls::entropy
// Provides: {"sys_read_entropy"}
// Dependencies: {}
# [doc = " Fill `len` bytes in `buf` with cryptographically secure random data."] # [doc = ""] # [doc = " Returns either the number of bytes written to buf (a positive value) or"] # [doc = " * `-EINVAL` if `flags` contains unknown flags."] # [doc = " * `-ENOSYS` if the system does not support random data generation."] # [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_read_entropy (buf : * mut u8 , len : usize , flags : u32) -> isize { unsafe { read_entropy (buf , len , flags) } }
};
}
