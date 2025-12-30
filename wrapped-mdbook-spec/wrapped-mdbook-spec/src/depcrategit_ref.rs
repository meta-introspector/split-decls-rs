// Generated macro for git_ref (function)
macro_rules! Depcrategit_ref {
() => {
// Module: crate
// Provides: {"git_ref"}
// Dependencies: {}
# [doc = " Determines the git ref used for linking to a particular branch/tag in GitHub."] fn git_ref (rust_root : & Option < PathBuf >) -> Result < String > { let Some (rust_root) = rust_root else { return Ok ("master" . into ()) ; } ; let channel = std :: fs :: read_to_string (rust_root . join ("src/ci/channel")) . context ("failed to read src/ci/channel") ? ; let git_ref = match channel . trim () { "nightly" => "master" . into () , "beta" => "beta" . into () , "stable" => { let version = std :: fs :: read_to_string (rust_root . join ("src/version")) . context ("|| failed to read src/version") ? ; version . trim () . into () } ch => bail ! ("unknown channel {ch}") , } ; Ok (git_ref) }
};
}
