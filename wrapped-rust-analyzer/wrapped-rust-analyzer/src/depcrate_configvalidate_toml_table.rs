// Generated macro for validate_toml_table (function)
macro_rules! Depcrate_configvalidate_toml_table {
() => {
// Module: crate::config
// Provides: {"validate_toml_table"}
// Dependencies: {}
fn validate_toml_table (known_ptrs : & [& [& 'static str]] , toml : & toml :: Table , ptr : & mut String , error_sink : & mut Vec < (String , toml :: de :: Error) > ,) { let verify = | ptr : & String | known_ptrs . iter () . any (| ptrs | ptrs . contains (& ptr . as_str ())) ; let l = ptr . len () ; for (k , v) in toml { if ! ptr . is_empty () { ptr . push ('_') ; } ptr . push_str (k) ; match v { toml :: Value :: Table (_) if verify (ptr) => () , toml :: Value :: Table (table) => validate_toml_table (known_ptrs , table , ptr , error_sink) , _ if ! verify (ptr) => error_sink . push ((ptr . replace ('_' , "/") , toml :: de :: Error :: custom ("unexpected field"))) , _ => () , } ptr . truncate (l) ; } }
};
}
