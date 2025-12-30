// Generated macro for tests (module)
macro_rules! Depcrate___macros_module_infotests {
() => {
// Module: crate::__macros::module_info
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] # [cfg_attr (not (all (target_vendor = "apple" , target_os = "macos" , target_arch = "x86")) , ignore = "Only relevant on macOS 32-bit")] fn ensure_size_of_module_info () { assert_eq ! (core :: mem :: size_of ::< ModuleInfo > () , 16) ; } }
};
}
