// Generated macro for Msrv (struct)
macro_rules! Depcrate_msrvsMsrv {
() => {
// Module: crate::msrvs
// Provides: {"Msrv"}
// Dependencies: {}
# [doc = " Tracks the current MSRV from `clippy.toml`, `Cargo.toml` or set via `#[clippy::msrv]` in late"] # [doc = " lint passes, use [`MsrvStack`] for early passes"] # [derive (Copy , Clone , Debug , Default)] pub struct Msrv (Option < RustcVersion >) ;
};
}
