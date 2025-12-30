// Generated macro for add_helper_macros (function)
macro_rules! Depcrate_arbitrary_in_macro_rulesadd_helper_macros {
() => {
// Module: crate::arbitrary_in_macro_rules
// Provides: {"add_helper_macros"}
// Dependencies: {}
fn add_helper_macros (file : & mut File , combinations : & std :: collections :: BTreeSet < Vec < String > >) { let mut insert_index = 0 ; for (i , item) in file . items . iter () . enumerate () { match item { Item :: Use (_) => { insert_index = i + 1 ; } Item :: Fn (_) => { break ; } _ => { } } } for traits in combinations { if traits . contains (& "Arbitrary" . to_string ()) { let macro_name = generate_macro_name (traits) ; let helper_macro = create_helper_macro (& macro_name , traits) ; file . items . insert (insert_index , helper_macro) ; insert_index += 1 ; } } }
};
}
