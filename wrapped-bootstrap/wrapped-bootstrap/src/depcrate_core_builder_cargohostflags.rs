// Generated macro for HostFlags (struct)
macro_rules! Depcrate_core_builder_cargoHostFlags {
() => {
// Module: crate::core::builder::cargo
// Provides: {"HostFlags"}
// Dependencies: {}
# [doc = " Flags that are passed to the `rustc` shim binary. These flags will only be applied when"] # [doc = " compiling host code, i.e. when `--target` is unset."] # [derive (Debug , Default)] struct HostFlags { rustc : Vec < String > , }
};
}
