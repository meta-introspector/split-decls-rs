// Generated macro for boringssl (module)
macro_rules! Depcrateboringssl {
() => {
// Module: crate
// Provides: {"boringssl"}
// Dependencies: {}
# [cfg (boringssl)] # [path = "."] mod boringssl { # [cfg (feature = "unstable_boringssl")] pub use bssl_sys :: * ; # [cfg (not (feature = "unstable_boringssl"))] include ! (concat ! (env ! ("OUT_DIR") , "/bindgen.rs")) ; pub fn init () { } }
};
}
