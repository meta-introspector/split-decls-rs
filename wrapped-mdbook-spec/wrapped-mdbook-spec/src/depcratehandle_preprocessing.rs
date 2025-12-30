// Generated macro for handle_preprocessing (function)
macro_rules! Depcratehandle_preprocessing {
() => {
// Module: crate
// Provides: {"handle_preprocessing"}
// Dependencies: {}
pub fn handle_preprocessing () -> Result < () , Error > { let pre = Spec :: new (None) ? ; let (ctx , book) = mdbook_preprocessor :: parse_input (io :: stdin ()) ? ; let book_version = Version :: parse (& ctx . mdbook_version) ? ; let version_req = VersionReq :: parse (mdbook_preprocessor :: MDBOOK_VERSION) ? ; if ! version_req . matches (& book_version) { eprintln ! ("warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}" , pre . name () , mdbook_preprocessor :: MDBOOK_VERSION , ctx . mdbook_version) ; } let processed_book = pre . run (& ctx , book) ? ; serde_json :: to_writer (io :: stdout () , & processed_book) ? ; Ok (()) }
};
}
