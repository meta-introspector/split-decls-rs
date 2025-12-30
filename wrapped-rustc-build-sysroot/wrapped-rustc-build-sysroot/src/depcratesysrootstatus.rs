// Generated macro for SysrootStatus (enum)
macro_rules! DepcrateSysrootStatus {
() => {
// Module: crate
// Provides: {"SysrootStatus"}
// Dependencies: {}
# [doc = " Whether a successful [`SysrootBuilder::build_from_source`] call found a cached sysroot or"] # [doc = " built a fresh one."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum SysrootStatus { # [doc = " The required sysroot is already cached."] AlreadyCached , # [doc = " A fresh sysroot was just compiled."] SysrootBuilt , }
};
}
