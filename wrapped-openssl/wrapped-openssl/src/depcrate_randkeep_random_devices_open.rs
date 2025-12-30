// Generated macro for keep_random_devices_open (function)
macro_rules! Depcrate_randkeep_random_devices_open {
() => {
// Module: crate::rand
// Provides: {"keep_random_devices_open"}
// Dependencies: {}
# [doc = " Controls random device file descriptor behavior."] # [doc = ""] # [doc = " Requires OpenSSL 1.1.1 or newer."] # [corresponds (RAND_keep_random_devices_open)] # [cfg (ossl111)] pub fn keep_random_devices_open (keep : bool) { unsafe { ffi :: RAND_keep_random_devices_open (keep as LenType) ; } }
};
}
