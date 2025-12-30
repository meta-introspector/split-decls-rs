// Generated macro for MsrvStack (struct)
macro_rules! Depcrate_msrvsMsrvStack {
() => {
// Module: crate::msrvs
// Provides: {"MsrvStack"}
// Dependencies: {}
# [doc = " Tracks the current MSRV from `clippy.toml`, `Cargo.toml` or set via `#[clippy::msrv]` in early"] # [doc = " lint passes, use [`Msrv`] for late passes"] # [derive (Debug , Clone)] pub struct MsrvStack { stack : SmallVec < [RustcVersion ; 2] > , }
};
}
