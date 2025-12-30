// Generated macro for cargo_config_file_reader_works (function)
macro_rules! Depcrate_cargo_config_filecargo_config_file_reader_works {
() => {
// Module: crate::cargo_config_file
// Provides: {"cargo_config_file_reader_works"}
// Dependencies: {}
# [test] fn cargo_config_file_reader_works () { # [cfg (target_os = "windows")] let root = "C://ROOT" ; # [cfg (not (target_os = "windows"))] let root = "/ROOT" ; let toml = format ! (r##"
alias.foo = "abc"
alias.bar = "🙂" # {root}/home/.cargo/config.toml
alias.sub-example = [
    "sub", # {root}/foo/.cargo/config.toml
    "example", # {root}/❤️💛💙/💝/.cargo/config.toml
]
build.rustflags = [
    "--flag", # {root}/home/.cargo/config.toml
    "env", # environment variable `CARGO_BUILD_RUSTFLAGS`
    "cli", # --config cli option
]
env.CARGO_WORKSPACE_DIR.relative = true # {root}/home/.cargo/config.toml
env.CARGO_WORKSPACE_DIR.value = "" # {root}/home/.cargo/config.toml
"##) ; let reader = CargoConfigFileReader :: new (& toml) . unwrap () ; let alias_foo = reader . get_spanned (["alias" , "foo"]) . unwrap () ; assert_eq ! (alias_foo . as_ref () . as_str () . unwrap () , "abc") ; assert ! (reader . get_origin_root (alias_foo) . is_none ()) ; let alias_bar = reader . get_spanned (["alias" , "bar"]) . unwrap () ; assert_eq ! (alias_bar . as_ref () . as_str () . unwrap () , "🙂") ; assert_eq ! (reader . get_origin_root (alias_bar) . unwrap () . as_str () , format ! ("{root}/home")) ; let alias_sub_example = reader . get_spanned (["alias" , "sub-example"]) . unwrap () ; assert ! (reader . get_origin_root (alias_sub_example) . is_none ()) ; let alias_sub_example = alias_sub_example . as_ref () . as_array () . unwrap () ; assert_eq ! (alias_sub_example [0] . get_ref () . as_str () . unwrap () , "sub") ; assert_eq ! (reader . get_origin_root (& alias_sub_example [0]) . unwrap () . as_str () , format ! ("{root}/foo")) ; assert_eq ! (alias_sub_example [1] . get_ref () . as_str () . unwrap () , "example") ; assert_eq ! (reader . get_origin_root (& alias_sub_example [1]) . unwrap () . as_str () , format ! ("{root}/❤️💛💙/💝")) ; let build_rustflags = reader . get (["build" , "rustflags"]) . unwrap () . as_array () . unwrap () ; assert_eq ! (reader . get_origin_root (& build_rustflags [0]) . unwrap () . as_str () , format ! ("{root}/home")) ; assert ! (reader . get_origin_root (& build_rustflags [1]) . is_none ()) ; assert ! (reader . get_origin_root (& build_rustflags [2]) . is_none ()) ; let env_cargo_workspace_dir = reader . get (["env" , "CARGO_WORKSPACE_DIR"]) . unwrap () . as_table () . unwrap () ; let env_relative = & env_cargo_workspace_dir ["relative"] ; assert ! (env_relative . as_ref () . as_bool () . unwrap ()) ; assert_eq ! (reader . get_origin_root (env_relative) . unwrap () . as_str () , format ! ("{root}/home")) ; let env_val = & env_cargo_workspace_dir ["value"] ; assert_eq ! (env_val . as_ref () . as_str () . unwrap () , "") ; assert_eq ! (reader . get_origin_root (env_val) . unwrap () . as_str () , format ! ("{root}/home")) ; }
};
}
