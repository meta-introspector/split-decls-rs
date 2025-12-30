// Generated macro for cargo_cfg (function)
macro_rules! Depcrate_inputcargo_cfg {
() => {
// Module: crate::input
// Provides: {"cargo_cfg"}
// Dependencies: {}
# [doc = " For each [configuration option] of the package being built, this will contain"] # [doc = " the value of the configuration."] # [doc = ""] # [doc = " This includes values built-in to the compiler"] # [doc = " (which can be seen with `rustc --print=cfg`) and values set by build scripts"] # [doc = " and extra flags passed to rustc (such as those defined in `RUSTFLAGS`)."] # [doc = ""] # [doc = " [configuration option]: https://doc.rust-lang.org/stable/reference/conditional-compilation.html"] # [track_caller] pub fn cargo_cfg (cfg : & str) -> Option < Vec < String > > { let var = cargo_cfg_var (cfg) ; ENV . get (& var) . map (| v | to_strings (v , ',')) }
};
}
