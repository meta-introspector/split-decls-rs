// Generated macro for black_box (function)
macro_rules! Depcrateblack_box {
() => {
// Module: crate
// Provides: {"black_box"}
// Dependencies: {}
# [doc = " A function that is opaque to the optimizer, used to prevent the compiler from"] # [doc = " optimizing away computations in a benchmark."] # [doc = ""] # [doc = " This variant is stable-compatible, but it may cause some performance overhead"] # [doc = " or fail to prevent code from being eliminated."] # [cfg (not (feature = "real_blackbox"))] pub fn black_box < T > (dummy : T) -> T { unsafe { let ret = std :: ptr :: read_volatile (& dummy) ; std :: mem :: forget (dummy) ; ret } }
};
}
