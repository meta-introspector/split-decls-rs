// Generated macro for remove_rustc_src_from_project (function)
macro_rules! Depcrate_setup_intellijremove_rustc_src_from_project {
() => {
// Module: crate::setup::intellij
// Provides: {"remove_rustc_src_from_project"}
// Dependencies: {}
fn remove_rustc_src_from_project (project : & ClippyProjectInfo) -> bool { let Ok (mut cargo_content) = read_project_file (project . cargo_file) else { return false ; } ; let Some (section_start) = cargo_content . find (RUSTC_PATH_SECTION) else { println ! ("info: dependencies could not be found in `{}` for {}, skipping file" , project . cargo_file , project . name) ; return true ; } ; let Some (end_point) = cargo_content . find (DEPENDENCIES_SECTION) else { eprintln ! ("error: the end of the rustc dependencies section could not be found in `{}`" , project . cargo_file) ; return false ; } ; cargo_content . replace_range (section_start .. end_point , "") ; match File :: create (project . cargo_file) { Ok (mut file) => { file . write_all (cargo_content . as_bytes ()) . unwrap () ; println ! ("info: successfully removed dependencies inside {}" , project . cargo_file) ; true } , Err (err) => { eprintln ! ("error: unable to open file `{}` to remove rustc dependencies for {} ({err})" , project . cargo_file , project . name) ; false } , } }
};
}
