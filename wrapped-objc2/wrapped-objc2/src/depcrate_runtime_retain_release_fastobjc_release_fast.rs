// Generated macro for objc_release_fast (function)
macro_rules! Depcrate_runtime_retain_release_fastobjc_release_fast {
() => {
// Module: crate::runtime::retain_release_fast
// Provides: {"objc_release_fast"}
// Dependencies: {}
# [doc = " A potentially faster version of `ffi::objc_release`."] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Same as `ffi::objc_release`."] # [inline] pub (crate) unsafe fn objc_release_fast (obj : * mut AnyObject) { # [cfg (all (feature = "unstable-apple-new" , target_arch = "aarch64"))] unsafe { core :: arch :: asm ! ("bl _objc_release_{obj:x}" , obj = in (reg) obj , out ("x16") _ , out ("x17") _ , out ("x30") _ , clobber_abi ("C") ,) } # [cfg (not (all (feature = "unstable-apple-new" , target_arch = "aarch64")))] unsafe { crate :: ffi :: objc_release (obj) } }
};
}
