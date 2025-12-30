// Generated macro for mini_tests (function)
macro_rules! Depcrate_testmini_tests {
() => {
// Module: crate::test
// Provides: {"mini_tests"}
// Dependencies: {}
fn mini_tests (env : & Env , args : & TestArg) -> Result < () , String > { println ! ("[BUILD] mini_core") ; let crate_types = if args . config_info . host_triple != args . config_info . target_triple { "lib" } else { "lib,dylib" } . to_string () ; let mut command = args . config_info . rustc_command_vec () ; command . extend_from_slice (& [& "example/mini_core.rs" , & "--crate-name" , & "mini_core" , & "--crate-type" , & crate_types , & "--target" , & args . config_info . target_triple ,]) ; run_command_with_output_and_env (& command , None , Some (env)) ? ; println ! ("[BUILD] example") ; let mut command = args . config_info . rustc_command_vec () ; command . extend_from_slice (& [& "example/example.rs" , & "--crate-type" , & "lib" , & "--target" , & args . config_info . target_triple ,]) ; run_command_with_output_and_env (& command , None , Some (env)) ? ; println ! ("[AOT] mini_core_hello_world") ; let mut command = args . config_info . rustc_command_vec () ; command . extend_from_slice (& [& "example/mini_core_hello_world.rs" , & "--crate-name" , & "mini_core_hello_world" , & "--crate-type" , & "bin" , & "-g" , & "--target" , & args . config_info . target_triple ,]) ; run_command_with_output_and_env (& command , None , Some (env)) ? ; let command : & [& dyn AsRef < OsStr >] = & [& Path :: new (& args . config_info . cargo_target_dir) . join ("mini_core_hello_world") , & "abc" , & "bcd" ,] ; maybe_run_command_in_vm (command , env , args) ? ; Ok (()) }
};
}
