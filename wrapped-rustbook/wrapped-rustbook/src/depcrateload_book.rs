// Generated macro for load_book (function)
macro_rules! Depcrateload_book {
() => {
// Module: crate
// Provides: {"load_book"}
// Dependencies: {}
fn load_book (book_dir : & Path) -> Result3 < MDBook > { let mut book = MDBook :: load (book_dir) ? ; book . config . set ("output.html.input-404" , "") . unwrap () ; book . config . set ("output.html.hash-files" , true) . unwrap () ; Ok (book) }
};
}
