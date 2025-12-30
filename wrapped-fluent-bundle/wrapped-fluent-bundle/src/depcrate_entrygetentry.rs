// Generated macro for GetEntry (trait)
macro_rules! Depcrate_entryGetEntry {
() => {
// Module: crate::entry
// Provides: {"GetEntry"}
// Dependencies: {}
pub trait GetEntry { # [doc = " Looks up a message by its string ID, and returns it if it exists."] fn get_entry_message (& self , id : & str) -> Option < & ast :: Message < & str > > ; # [doc = " Looks up a term by its string ID, and returns it if it exists."] fn get_entry_term (& self , id : & str) -> Option < & ast :: Term < & str > > ; # [doc = " Looks up a function by its string ID, and returns it if it exists."] fn get_entry_function (& self , id : & str) -> Option < & FluentFunction > ; }
};
}
