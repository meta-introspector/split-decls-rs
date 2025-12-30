// Generated macro for Columns (struct)
macro_rules! Depcrate_output_tableColumns {
() => {
// Module: crate::output::table
// Provides: {"Columns"}
// Dependencies: {}
# [doc = " Extra columns to display in the table."] # [allow (clippy :: struct_excessive_bools)] # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub struct Columns { # [doc = " At least one of these timestamps will be shown."] pub time_types : TimeTypes , pub inode : bool , pub links : bool , pub blocksize : bool , pub group : bool , pub git : bool , pub subdir_git_repos : bool , pub subdir_git_repos_no_stat : bool , pub octal : bool , pub security_context : bool , pub file_flags : bool , pub permissions : bool , pub filesize : bool , pub user : bool , }
};
}
