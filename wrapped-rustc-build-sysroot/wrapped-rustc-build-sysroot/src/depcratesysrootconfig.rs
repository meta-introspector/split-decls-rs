// Generated macro for SysrootConfig (enum)
macro_rules! DepcrateSysrootConfig {
() => {
// Module: crate
// Provides: {"SysrootConfig"}
// Dependencies: {}
# [doc = " Settings controlling how the sysroot will be built."] # [derive (Clone , Debug , PartialEq , Eq , Hash)] pub enum SysrootConfig { # [doc = " Build a no-std (only core and alloc) sysroot."] NoStd , # [doc = " Build a full sysroot with the `std` and `test` crates."] WithStd { # [doc = " Features to enable for the `std` crate."] std_features : Vec < String > , } , }
};
}
