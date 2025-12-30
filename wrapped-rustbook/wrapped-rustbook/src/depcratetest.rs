// Generated macro for test (function)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
fn test (args : & ArgMatches) -> Result3 < () > { let book_dir = get_book_dir (args) ; let library_paths = args . try_get_one :: < Vec < String > > ("library-path") ? . map (| v | v . iter () . map (| s | s . as_str ()) . collect :: < Vec < & str > > ()) . unwrap_or_default () ; let mut book = load_book (& book_dir) ? ; book . test (library_paths) }
};
}
