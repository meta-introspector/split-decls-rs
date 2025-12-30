// Generated macro for lseek64 (function)
macro_rules! Depcrate_gzlseek64 {
() => {
// Module: crate::gz
// Provides: {"lseek64"}
// Dependencies: {}
fn lseek64 (fd : c_int , offset : z_off64_t , origin : c_int) -> z_off64_t { # [cfg (any (target_os = "linux" , target_os = "android" , target_os = "windows"))] { return unsafe { libc :: lseek64 (fd , offset as _ , origin) as z_off64_t } ; } # [allow (unused)] { (unsafe { libc :: lseek (fd , offset as _ , origin) }) as z_off64_t } }
};
}
