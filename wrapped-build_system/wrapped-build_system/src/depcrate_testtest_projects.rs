// Generated macro for test_projects (function)
macro_rules! Depcrate_testtest_projects {
() => {
// Module: crate::test
// Provides: {"test_projects"}
// Dependencies: {}
fn test_projects (env : & Env , args : & TestArg) -> Result < () , String > { let projects = ["https://github.com/rust-random/getrandom" , "https://github.com/BurntSushi/memchr" , "https://github.com/dtolnay/itoa" , "https://github.com/rust-lang/cfg-if" , "https://github.com/rust-lang/log" , "https://github.com/bitflags/bitflags" ,] ; let mut env = env . clone () ; let rustflags = format ! ("{} --cap-lints allow" , env . get ("RUSTFLAGS") . cloned () . unwrap_or_default ()) ; env . insert ("RUSTFLAGS" . to_string () , rustflags) ; let run_tests = | projects_path , iter : & mut dyn Iterator < Item = & & str > | -> Result < () , String > { for project in iter { let clone_result = git_clone_root_dir (project , projects_path , true) ? ; let repo_path = Path :: new (& clone_result . repo_dir) ; run_cargo_command (& [& "build" , & "--release"] , Some (repo_path) , & env , args) ? ; run_cargo_command (& [& "test"] , Some (repo_path) , & env , args) ? ; } Ok (()) } ; let projects_path = Path :: new ("projects") ; create_dir (projects_path) ? ; let nb_parts = args . nb_parts . unwrap_or (0) ; if nb_parts > 0 { let count = projects . len () / nb_parts + 1 ; let current_part = args . current_part . unwrap () ; let start = current_part * count ; run_tests (projects_path , & mut projects . iter () . skip (start) . take (count)) ? ; } else { run_tests (projects_path , & mut projects . iter ()) ? ; } Ok (()) }
};
}
