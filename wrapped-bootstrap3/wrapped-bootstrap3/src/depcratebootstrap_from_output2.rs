// Generated macro for bootstrap_from_output2 (function)
macro_rules! Depcratebootstrap_from_output2 {
() => {
// Module: crate
// Provides: {"bootstrap_from_output2"}
// Dependencies: {}
pub fn bootstrap_from_output2 (output2_dir : & Path , output3_dir : & Path) -> Result < () , Box < dyn std :: error :: Error > > { println ! ("🚀 Bootstrap3: Proving comprehensive macro wrapping!") ; wrapped_fs_create_dir_all ! (output3_dir) ? ; for entry in wrapped_fs_read_dir ! (output2_dir) ? { let entry = entry ? ; if entry . file_type () ? . is_dir () { let crate_path = entry . path () ; if let Err (e) = process_crate (& crate_path , output3_dir) { println ! ("⚠️  Skipped {}: {}" , crate_path . display () , e) ; } } } println ! ("🎉 Bootstrap3 complete with ALL WRAPPED macros!") ; Ok (()) }
};
}
