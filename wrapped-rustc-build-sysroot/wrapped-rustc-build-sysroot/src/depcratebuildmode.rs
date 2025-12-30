// Generated macro for BuildMode (enum)
macro_rules! DepcrateBuildMode {
() => {
// Module: crate
// Provides: {"BuildMode"}
// Dependencies: {}
# [doc = " The build mode to use for this sysroot."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum BuildMode { # [doc = " Do a full sysroot build. Suited for all purposes (like the regular sysroot), but only works"] # [doc = " for the host or for targets that have suitable development tools installed."] Build , # [doc = " Do a check-only sysroot build. This is only suited for check-only builds of crates, but on"] # [doc = " the plus side it works for *arbitrary* targets without having any special tools installed."] Check , }
};
}
