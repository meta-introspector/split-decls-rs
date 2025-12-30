// Generated macro for write_lib_cargo_toml (function)
macro_rules! Depcrate_common_gen_rustwrite_lib_cargo_toml {
() => {
// Module: crate::common::gen_rust
// Provides: {"write_lib_cargo_toml"}
// Dependencies: {}
pub fn write_lib_cargo_toml (w : & mut impl std :: io :: Write , name : & str) -> std :: io :: Result < () > { write_cargo_toml_header (w , name) ? ; writeln ! (w , "[dependencies]") ? ; writeln ! (w , "core_arch = {{ path = \"../../crates/core_arch\" }}") ? ; Ok (()) }
};
}
