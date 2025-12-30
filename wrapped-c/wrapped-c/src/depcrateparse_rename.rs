// Generated macro for parse_rename (function)
macro_rules! Depcrateparse_rename {
() => {
// Module: crate
// Provides: {"parse_rename"}
// Dependencies: {}
# [cfg (feature = "clap")] fn parse_rename (name : & str) -> Result < (String , String) > { let mut parts = name . splitn (2 , '=') ; let to_rename = parts . next () . unwrap () ; match parts . next () { Some (part) => Ok ((to_rename . to_string () , part . to_string ())) , None => anyhow :: bail ! ("`--rename` option must have an `=` in it (e.g. `--rename a=b`)") , } }
};
}
