// Generated macro for execProc (function)
macro_rules! Depcrate_processexecProc {
() => {
// Module: crate::process
// Provides: {"execProc"}
// Dependencies: {}
fn execProc (command : Command , startup_info : STARTUPINFOEXW) -> win :: Result < PROCESS_INFORMATION > { let commandline = build_commandline (& command) ; let mut commandline = convert_osstr_to_utf16 (& commandline) ; let commandline = PWSTR (commandline . as_mut_ptr ()) ; let current_dir = command . get_current_dir () ; let current_dir = current_dir . map (| p | convert_osstr_to_utf16 (p . as_os_str ())) ; let current_dir = current_dir . as_ref () . map_or (null () , | dir | dir . as_ptr ()) ; let current_dir = PCWSTR (current_dir) ; let envs_list = | | { command . get_envs () . filter_map (| (key , value) | value . map (| value | (key , value))) } ; let envs = environment_block_unicode (envs_list ()) ; let envs = if envs_list () . next () . is_some () { Some (envs . as_ptr () as _) } else { None } ; let appname = PCWSTR (null_mut ()) ; let dwflags = EXTENDED_STARTUPINFO_PRESENT | CREATE_UNICODE_ENVIRONMENT ; let mut proc_info = PROCESS_INFORMATION :: default () ; unsafe { CreateProcessW (appname , commandline , None , None , false , dwflags , envs , current_dir , & startup_info . StartupInfo , & mut proc_info ,) ? } ; Ok (proc_info) }
};
}
