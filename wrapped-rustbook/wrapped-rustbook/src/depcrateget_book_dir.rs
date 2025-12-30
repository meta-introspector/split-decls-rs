// Generated macro for get_book_dir (function)
macro_rules! Depcrateget_book_dir {
() => {
// Module: crate
// Provides: {"get_book_dir"}
// Dependencies: {}
fn get_book_dir (args : & ArgMatches) -> PathBuf { if let Some (p) = args . get_one :: < PathBuf > ("dir") { if p . is_relative () { env :: current_dir () . unwrap () . join (p) } else { p . to_path_buf () } } else { env :: current_dir () . unwrap () } }
};
}
