// Generated macro for objc_retain_fast (function)
macro_rules! Depcrate_runtime_retain_release_fastobjc_retain_fast {
() => {
// Module: crate::runtime::retain_release_fast
// Provides: {"objc_retain_fast"}
// Dependencies: {}
# [doc = " A potentially faster version of `ffi::objc_retain`."] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Same as `ffi::objc_retain`."] # [inline] pub (crate) unsafe fn objc_retain_fast (obj : * mut AnyObject) -> * mut AnyObject { # [cfg (all (feature = "unstable-apple-new" , target_arch = "aarch64"))] unsafe { let result ; core :: arch :: asm ! ("bl _objc_retain_{obj:x}" , obj = in (reg) obj , lateout ("x0") result , out ("x16") _ , out ("x17") _ , out ("x30") _ , clobber_abi ("C") ,) ; result } # [cfg (not (all (feature = "unstable-apple-new" , target_arch = "aarch64")))] unsafe { crate :: ffi :: objc_retain (obj) } }
};
}
