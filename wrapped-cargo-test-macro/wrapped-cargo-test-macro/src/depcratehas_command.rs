// Generated macro for has_command (function)
macro_rules! Depcratehas_command {
() => {
// Module: crate
// Provides: {"has_command"}
// Dependencies: {}
fn has_command (command : & str) -> bool { use std :: env :: consts :: EXE_EXTENSION ; # [allow (clippy :: disallowed_methods)] let Some (paths) = std :: env :: var_os ("PATH") else { return false ; } ; std :: env :: split_paths (& paths) . flat_map (| path | { let candidate = path . join (& command) ; let with_exe = if EXE_EXTENSION . is_empty () { None } else { Some (candidate . with_extension (EXE_EXTENSION)) } ; std :: iter :: once (candidate) . chain (with_exe) }) . find (| p | is_executable (p)) . is_some () }
};
}
