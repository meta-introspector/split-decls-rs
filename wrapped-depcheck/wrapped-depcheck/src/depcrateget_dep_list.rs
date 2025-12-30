// Generated macro for get_dep_list (function)
macro_rules! Depcrateget_dep_list {
() => {
// Module: crate
// Provides: {"get_dep_list"}
// Dependencies: {}
# [doc = " Get the deep (fully resolved) dependency list produced by `cargo tree -p {package} -e {edge_kind}`"] fn get_dep_list (package : & str , edge_kind : & str , extra_args : & str) -> Vec < DepSpec > { let mut cmd = Command :: new ("cargo") ; cmd . arg ("tree") . arg ("-p") . arg (package) . arg ("-e") . arg (edge_kind) . arg ("--no-default-features") ; for arg in extra_args . split (' ') { if ! arg . is_empty () { cmd . arg (arg) ; } } let output = cmd . output () . expect ("Failed to run `cargo tree`") ; if ! output . status . success () { eprintln ! ("Failed to run `cargo tree -p {package} -e {edge_kind} --no-default-features {extra_args}`:") ; if let Ok (s) = str :: from_utf8 (& output . stderr) { eprintln ! ("{s}") ; } process :: exit (1) ; } let mut spec : Vec < _ > = output . stdout . split (| b | * b == b'\n') . filter_map (| slice | { if slice . is_empty () { return None ; } if slice [0] == b'[' { return None ; } let mut iter = slice . split (| b | * b == b' ') ; let mut found_crate_name = None ; for section in & mut iter { if section . is_empty () { continue ; } if char :: from (section [0]) . is_ascii_alphabetic () { found_crate_name = Some (str :: from_utf8 (section) . expect ("Must be utf-8") . to_owned ()) ; break ; } } if let Some (crate_name) = found_crate_name { let crate_version = iter . next () . expect ("There must be a version after the crate name!") ; let crate_version = str :: from_utf8 (crate_version) . expect ("Must be utf-8") . to_owned () ; Some (DepSpec { crate_name , crate_version , }) } else { None } }) . collect () ; spec . sort () ; spec . dedup () ; spec }
};
}
