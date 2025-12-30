// Generated macro for ffi (module)
macro_rules! Depcrateffi {
() => {
// Module: crate
// Provides: {"ffi"}
// Dependencies: {}
# [cxx :: bridge] mod ffi { # [doc = " Parameters that will be given to GenMC for setting up the model checker."] # [doc = " (The fields of this struct are visible to both Rust and C++)"] # [derive (Clone , Debug)] struct GenmcParams { pub print_random_schedule_seed : bool , pub do_symmetry_reduction : bool , } unsafe extern "C++" { include ! ("MiriInterface.hpp") ; type MiriGenMCShim ; fn createGenmcHandle (config : & GenmcParams) -> UniquePtr < MiriGenMCShim > ; } }
};
}
