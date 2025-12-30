// Generated macro for write_bin_cargo_toml (function)
macro_rules! Depcrate_common_gen_rustwrite_bin_cargo_toml {
() => {
// Module: crate::common::gen_rust
// Provides: {"write_bin_cargo_toml"}
// Dependencies: {}
pub fn write_bin_cargo_toml (w : & mut impl std :: io :: Write , module_count : usize ,) -> std :: io :: Result < () > { write_cargo_toml_header (w , "intrinsic-test-programs") ? ; writeln ! (w , "[dependencies]") ? ; writeln ! (w , "core_arch = {{ path = \"../crates/core_arch\" }}") ? ; for i in 0 .. module_count { writeln ! (w , "mod_{i} = {{ path = \"mod_{i}/\" }}") ? ; } Ok (()) }
};
}
