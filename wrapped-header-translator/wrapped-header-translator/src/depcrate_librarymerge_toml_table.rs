// Generated macro for merge_toml_table (function)
macro_rules! Depcrate_librarymerge_toml_table {
() => {
// Module: crate::library
// Provides: {"merge_toml_table"}
// Dependencies: {}
fn merge_toml_table (original : & mut Table , addition : Table) { for (key , mut addition) in addition { match original . entry (& key) { toml_edit :: Entry :: Occupied (mut original) => match (original . get_mut () , addition) { (Item :: Value (original) , Item :: Value (addition)) => { * original = addition ; } (Item :: Table (original) , Item :: Table (addition)) => { merge_toml_table (original , addition) ; } (Item :: ArrayOfTables (original) , Item :: ArrayOfTables (addition)) => { * original = addition ; } (original , addition) => { * original = addition ; } } , toml_edit :: Entry :: Vacant (original) => { match & mut addition { Item :: Table (table) => { table . set_position (isize :: MAX) ; table . decor_mut () . clear () ; } Item :: ArrayOfTables (array) => { for table in array . iter_mut () { table . set_position (isize :: MAX) ; table . decor_mut () . clear () ; } } _ => { } } original . insert (addition) ; } } } }
};
}
