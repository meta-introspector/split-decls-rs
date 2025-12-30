// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl Parse for Map { fn parse (input : ParseStream < '_ >) -> parse :: Result < Map > { let parsed = Punctuated :: < Entry , Token ! [,] > :: parse_terminated (input) ? ; let mut expanded_entries = Vec :: new () ; for entry in parsed { for (i , (parsed_key , expr)) in entry . key . parsed . iter () . zip (entry . key . expr . iter ()) . enumerate () { let expanded_key = Key { parsed : vec ! [parsed_key . clone ()] , expr : vec ! [expr . clone ()] , attrs : if i == 0 { entry . key . attrs . clone () } else { Vec :: new () } , } ; let expanded_entry = Entry { key : expanded_key , value : entry . value . clone () , attrs : if i == 0 { entry . attrs . clone () } else { Vec :: new () } , } ; expanded_entries . push (expanded_entry) ; } } check_duplicates (& expanded_entries) ? ; Ok (Map (expanded_entries)) } }
};
}
