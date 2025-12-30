// Generated macro for Config (struct)
macro_rules! DepcrateConfig {
() => {
// Module: crate
// Provides: {"Config"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct Config { statik : Option < bool > , min_version : Bound < String > , max_version : Bound < String > , extra_args : Vec < OsString > , cargo_metadata : bool , env_metadata : bool , print_system_libs : bool , print_system_cflags : bool , }
};
}
