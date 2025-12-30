// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl Parse for Set { fn parse (input : ParseStream < '_ >) -> parse :: Result < Set > { let parsed = Punctuated :: < Key , Token ! [,] > :: parse_terminated (input) ? ; let unit_value : Expr = syn :: parse_str ("()") . expect ("Failed to parse unit value") ; let mut expanded_entries = Vec :: new () ; for key in parsed { for (i , (parsed_key , expr)) in key . parsed . iter () . zip (key . expr . iter ()) . enumerate () { let expanded_key = Key { parsed : vec ! [parsed_key . clone ()] , expr : vec ! [expr . clone ()] , attrs : if i == 0 { key . attrs . clone () } else { Vec :: new () } , } ; let expanded_entry = Entry { key : expanded_key , value : unit_value . clone () , attrs : if i == 0 { key . attrs . clone () } else { Vec :: new () } , } ; expanded_entries . push (expanded_entry) ; } } check_duplicates (& expanded_entries) ? ; Ok (Set (expanded_entries)) } }
};
}
