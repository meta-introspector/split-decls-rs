// Generated macro for cvt (function)
macro_rules! Depcrate_preview1cvt {
() => {
// Module: crate::preview1
// Provides: {"cvt"}
// Dependencies: {}
fn cvt (err : i32) -> i32 { match err { libc :: EINVAL => ERRNO_INVAL . raw () as i32 , libc :: EFAULT => ERRNO_FAULT . raw () as i32 , libc :: ENOMEM => ERRNO_NOMEM . raw () as i32 , _ => ERRNO_NOSYS . raw () as i32 , } }
};
}
