// Generated macro for MiriEnv (struct)
macro_rules! Depcrate_utilMiriEnv {
() => {
// Module: crate::util
// Provides: {"MiriEnv"}
// Dependencies: {}
# [doc = " Some extra state we track for building Miri, such as the right RUSTFLAGS."] pub struct MiriEnv { # [doc = " miri_dir is the root of the miri repository checkout we are working in."] pub miri_dir : PathBuf , # [doc = " active_toolchain is passed as `+toolchain` argument to cargo/rustc invocations."] toolchain : String , # [doc = " The cargo binary to use."] cargo_bin : String , # [doc = " Extra flags to pass to cargo."] cargo_extra_flags : Vec < String > , # [doc = " The rustc sysroot"] pub sysroot : PathBuf , # [doc = " The shell we use."] pub sh : Shell , # [doc = " The library dir in the sysroot."] pub libdir : PathBuf , }
};
}
