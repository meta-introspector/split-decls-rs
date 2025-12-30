// Generated macro for inject_deps_into_project (function)
macro_rules! Depcrate_setup_intellijinject_deps_into_project {
() => {
// Module: crate::setup::intellij
// Provides: {"inject_deps_into_project"}
// Dependencies: {}
fn inject_deps_into_project (rustc_source_dir : & Path , project : & ClippyProjectInfo) -> Result < () , () > { let cargo_content = read_project_file (project . cargo_file) ? ; let lib_content = read_project_file (project . lib_rs_file) ? ; if inject_deps_into_manifest (rustc_source_dir , project . cargo_file , & cargo_content , & lib_content) . is_err () { eprintln ! ("error: unable to inject dependencies into {} with the Cargo file {}" , project . name , project . cargo_file) ; Err (()) } else { Ok (()) } }
};
}
