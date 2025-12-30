// Generated macro for build (function)
macro_rules! Depcratebuild {
() => {
// Module: crate
// Provides: {"build"}
// Dependencies: {}
pub fn build (args : & ArgMatches) -> Result3 < () > { let book_dir = get_book_dir (args) ; let mut book = load_book (& book_dir) ? ; if let Some (lang) = args . get_one :: < String > ("lang") { let gettext = Gettext ; book . with_preprocessor (gettext) ; book . config . set ("book.language" , lang) . unwrap () ; } book . config . build . create_missing = false ; if let Some (dest_dir) = args . get_one :: < PathBuf > ("dest-dir") { book . config . build . build_dir = dest_dir . into () ; } if book . config . get_preprocessor ("trpl-note") . is_some () { book . with_preprocessor (Note) ; } if book . config . get_preprocessor ("trpl-listing") . is_some () { book . with_preprocessor (Listing) ; } if book . config . get_preprocessor ("trpl-figure") . is_some () { book . with_preprocessor (Figure) ; } if book . config . get_preprocessor ("spec") . is_some () { let rust_root = args . get_one :: < PathBuf > ("rust-root") . cloned () ; book . with_preprocessor (Spec :: new (rust_root) ?) ; } book . build () ? ; Ok (()) }
};
}
