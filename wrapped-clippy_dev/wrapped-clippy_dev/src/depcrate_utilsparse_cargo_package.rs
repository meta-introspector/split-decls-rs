// Generated macro for parse_cargo_package (function)
macro_rules! Depcrate_utilsparse_cargo_package {
() => {
// Module: crate::utils
// Provides: {"parse_cargo_package"}
// Dependencies: {}
# [must_use] pub fn parse_cargo_package (s : & str) -> CargoPackage < '_ > { let mut in_package = false ; let mut in_platform_deps = false ; let mut name = "" ; let mut version_range = 0 .. 0 ; let mut not_a_platform_range = 0 .. 0 ; for (offset , part) in toml_iter (s) { match part { TomlPart :: Table (name) => { if in_platform_deps { not_a_platform_range . end = offset ; } in_package = false ; in_platform_deps = false ; match name . trim () { "package" => in_package = true , "target.'cfg(NOT_A_PLATFORM)'.dependencies" => { in_platform_deps = true ; not_a_platform_range . start = offset ; } , _ => { } , } } , TomlPart :: Value (key , value) if in_package => match key . trim_end () { "name" => name = value . trim () , "version" => { version_range . start = offset + (value . len () - value . trim () . len ()) + key . len () + 1 ; version_range . end = offset + key . len () + value . trim_end () . len () + 1 ; } , _ => { } , } , TomlPart :: Value (..) => { } , } } CargoPackage { name , version_range , not_a_platform_range , } }
};
}
