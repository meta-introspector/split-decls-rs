// Generated macro for tests (module)
macro_rules! Depcrate_conftests {
() => {
// Module: crate::conf
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use serde :: de :: IgnoredAny ; use std :: collections :: { HashMap , HashSet } ; use std :: fs ; use walkdir :: WalkDir ; # [test] fn configs_are_tested () { let mut names : HashSet < String > = crate :: get_configuration_metadata () . into_iter () . filter_map (| meta | { if meta . deprecation_reason . is_none () { Some (meta . name . replace ('_' , "-")) } else { None } }) . collect () ; let toml_files = WalkDir :: new ("../tests") . into_iter () . map (Result :: unwrap) . filter (| entry | entry . file_name () == "clippy.toml") ; for entry in toml_files { let file = fs :: read_to_string (entry . path ()) . unwrap () ; # [expect (clippy :: zero_sized_map_values)] if let Ok (map) = toml :: from_str :: < HashMap < String , IgnoredAny > > (& file) { for name in map . keys () { names . remove (name . as_str ()) ; } } } assert ! (names . is_empty () , "Configuration variable lacks test: {names:?}\nAdd a test to `tests/ui-toml`") ; } }
};
}
