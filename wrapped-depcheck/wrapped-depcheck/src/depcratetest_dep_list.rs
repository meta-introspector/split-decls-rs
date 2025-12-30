// Generated macro for test_dep_list (function)
macro_rules! Depcratetest_dep_list {
() => {
// Module: crate
// Provides: {"test_dep_list"}
// Dependencies: {}
# [doc = " Given a `cargo tree` invocation and the dependency sets to check, checks for any unlisted or duplicated deps"] # [doc = ""] # [doc = " `dep_list_name_for_error` is the name of the const above to show in the error suggestion"] fn test_dep_list (package : & str , edge_kind : & str , extra_args : & str , sets : & [& BTreeSet < & str >] , dep_list_name_for_error : & str ,) { println ! ("Testing `cargo tree -p {package} -e {edge_kind} --no-default-features {extra_args}`") ; let mut errors = Vec :: new () ; let dep_list = get_dep_list (package , edge_kind , extra_args) ; for i in dep_list . windows (2) { if i [0] . crate_name == i [1] . crate_name { errors . push (format ! ("Found two versions for `{0}` ({1} & {2})" , i [0] . crate_name , i [0] . crate_version , i [1] . crate_version)) ; } } 'dep_loop : for i in dep_list { if i . crate_name == package { continue ; } let name = & i . crate_name ; for s in sets { if s . contains (& * * name) { continue 'dep_loop ; } } errors . push (format ! ("Found non-allowlisted crate `{name}`, consider adding to \
                             {dep_list_name_for_error} in tools/depcheck/src/allowlist.rs if intentional")) ; } if ! errors . is_empty () { eprintln ! ("Found invalid dependencies:") ; for e in errors { eprintln ! ("\t{e}") ; } process :: exit (1) ; } }
};
}
