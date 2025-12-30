// Generated macro for parse_output_cargo_config_env_works (function)
macro_rules! Depcrate_envparse_output_cargo_config_env_works {
() => {
// Module: crate::env
// Provides: {"parse_output_cargo_config_env_works"}
// Dependencies: {}
# [test] fn parse_output_cargo_config_env_works () { use itertools :: Itertools ; let cwd = paths :: AbsPathBuf :: try_from (paths :: Utf8PathBuf :: try_from (std :: env :: current_dir () . unwrap ()) . unwrap () ,) . unwrap () ; let config_path = cwd . join (".cargo") . join ("config.toml") ; let raw = r#"
env.CARGO_WORKSPACE_DIR.relative = true
env.CARGO_WORKSPACE_DIR.value = ""
env.INVALID.relative = "invalidbool"
env.INVALID.value = "../relative"
env.RELATIVE.relative = true
env.RELATIVE.value = "../relative"
env.TEST.value = "test"
env.FORCED.value = "test"
env.FORCED.force = true
env.UNFORCED.value = "test"
env.UNFORCED.forced = false
env.OVERWRITTEN.value = "test"
env.NOT_AN_OBJECT = "value"
"# ; let raw = raw . lines () . map (| l | format ! ("{l} # {config_path}")) . join ("\n") ; let config = CargoConfigFile :: from_string_for_test (raw) ; let extra_env = [("FORCED" , Some ("ignored")) , ("UNFORCED" , Some ("newvalue")) , ("OVERWRITTEN" , Some ("newvalue")) , ("TEST" , None) ,] . iter () . map (| (k , v) | (k . to_string () , v . map (ToString :: to_string))) . collect () ; let env = cargo_config_env (& Some (config) , & extra_env) ; assert_eq ! (env . get ("CARGO_WORKSPACE_DIR") . as_deref () , Some (cwd . join ("") . as_str ())) ; assert_eq ! (env . get ("RELATIVE") . as_deref () , Some (cwd . join ("../relative") . as_str ())) ; assert_eq ! (env . get ("INVALID") . as_deref () , Some ("../relative")) ; assert_eq ! (env . get ("TEST") . as_deref () , Some ("test")) ; assert_eq ! (env . get ("FORCED") . as_deref () , Some ("test")) ; assert_eq ! (env . get ("UNFORCED") . as_deref () , Some ("newvalue")) ; assert_eq ! (env . get ("OVERWRITTEN") . as_deref () , Some ("newvalue")) ; assert_eq ! (env . get ("NOT_AN_OBJECT") . as_deref () , Some ("value")) ; }
};
}
