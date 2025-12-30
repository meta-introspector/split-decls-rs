// Generated macro for print_index_entry_status (function)
macro_rules! Depcrate_repository_statusprint_index_entry_status {
() => {
// Module: crate::repository::status
// Provides: {"print_index_entry_status"}
// Dependencies: {}
fn print_index_entry_status (out : & mut dyn std :: io :: Write , prefix : & Path , rela_path : & BStr , status : EntryStatus < () , gix :: submodule :: Status > ,) -> std :: io :: Result < () > { let char_storage ; let status = match status { EntryStatus :: Conflict { summary , entries : _ } => as_str (summary) , EntryStatus :: Change (change) => { char_storage = change_to_char (& change) ; std :: str :: from_utf8 (std :: slice :: from_ref (& char_storage)) . expect ("valid ASCII") } EntryStatus :: NeedsUpdate (_stat) => { return Ok (()) ; } EntryStatus :: IntentToAdd => "A" , } ; let rela_path = gix :: path :: from_bstr (rela_path) ; let display_path = gix :: path :: relativize_with_prefix (& rela_path , prefix) ; writeln ! (out , "{status: >3} {}" , display_path . display ()) }
};
}
