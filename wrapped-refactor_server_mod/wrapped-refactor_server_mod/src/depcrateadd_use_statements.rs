// Generated macro for add_use_statements (function)
macro_rules! Depcrateadd_use_statements {
() => {
// Module: crate
// Provides: {"add_use_statements"}
// Dependencies: {}
# [doc = " Helper to add a use statement to a File, avoiding duplicates."] fn add_use_statements (file : & mut File , new_uses : Vec < syn :: ItemUse >) { let mut existing_uses : Vec < String > = file . items . iter () . filter_map (| item | { if let Item :: Use (item_use) = item { Some (quote ! { # item_use } . to_string ()) } else { None } }) . collect () ; for new_use in new_uses { let new_use_tree_str = quote ! { # new_use } . to_string () ; if ! existing_uses . contains (& new_use_tree_str) { file . items . insert (0 , Item :: Use (new_use)) ; existing_uses . insert (0 , new_use_tree_str) ; } } }
};
}
