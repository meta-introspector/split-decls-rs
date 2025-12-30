// Generated macro for impl_106 (impl)
macro_rules! Depcrate_test_linksimpl_106 {
() => {
// Module: crate::test_links
// Provides: {"impl_106"}
// Dependencies: {}
impl Spec { # [doc = " Scans all tests in rust-lang/rust, and creates a mapping of a rule"] # [doc = " identifier to the set of tests that include that identifier."] pub fn collect_tests (& self , rules : & Rules) -> RuleToTests { let mut map = HashMap :: new () ; let Some (rust_root) = & self . rust_root else { return map ; } ; for entry in WalkDir :: new (rust_root . join ("tests")) { let entry = entry . unwrap () ; let path = entry . path () ; let relative = path . strip_prefix (rust_root) . unwrap_or_else (| _ | { panic ! ("expected root {rust_root:?} to be a prefix of {path:?}") }) ; if path . extension () . unwrap_or_default () == "rs" { let contents = std :: fs :: read_to_string (path) . unwrap () ; for line in contents . lines () { if let Some (id) = line . strip_prefix ("//@ reference: ") { if rules . interior_prefixes . contains (id) { let instead : Vec < _ > = rules . def_paths . keys () . filter (| key | key . starts_with (& format ! ("{id}."))) . collect () ; eprintln ! ("info: Interior prefix rule {id} found in {path:?}\n    \
                                 Tests should not be annotated with prefixed rule names.\n    \
                                 Use the rules from {instead:?} instead.") ; } else if ! rules . def_paths . contains_key (id) { eprintln ! ("info: Orphaned rule identifier {id} found in {path:?}\n    \
                                 Please update the test to use an existing rule name.") ; } let test = Test { path : relative . to_str () . unwrap () . replace ('\\' , "/") , } ; map . entry (id . to_string ()) . or_default () . push (test) ; } } } } for tests in map . values_mut () { tests . sort_by (| a , b | a . path . cmp (& b . path)) ; } map } }
};
}
