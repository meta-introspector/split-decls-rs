// Generated macro for parse (function)
macro_rules! Depcrate_expand_pathparse {
() => {
// Module: crate::expand_path
// Provides: {"parse"}
// Dependencies: {}
# [doc = " Parse user information from the given `path`, returning `(possible user information, adjusted input path)`."] # [doc = ""] # [doc = " Supported formats for user extraction are…"] # [doc = " * `~/repopath` - the currently logged in user's home."] # [doc = " * `~user/repopath` - the repository in the given user's home."] pub fn parse (path : & BStr) -> Result < (Option < ForUser > , BString) , Error > { Ok (path_segments (path) . and_then (| mut iter | { iter . next () . map (| segment | { if segment . starts_with (b"~") { let eu = if segment . len () == 1 { Some (ForUser :: Current) } else { Some (ForUser :: Name (segment [1 ..] . into ())) } ; (eu , format ! ("/{}" , iter . map (| s | s . as_bstr () . to_str_lossy ()) . collect ::< Vec < _ >> () . join ("/")) . into () ,) } else { (None , path . into ()) } }) }) . unwrap_or_else (| | (None , path . into ()))) }
};
}
