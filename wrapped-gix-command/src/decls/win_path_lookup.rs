macro_rules! win_path_lookup {
    () => {
        # [doc = " Try to find `command` in the `path_value` (the value of `PATH`) as separated by `;`, or return `None`."] # [doc = " Has special handling for `.exe` extensions, as these will be appended automatically if needed."] # [doc = " Note that just like Git, no lookup is performed if a slash or backslash is in `command`."] fn win_path_lookup (command : & Path , path_value : & std :: ffi :: OsStr) -> Option < PathBuf > { fn lookup (root : & bstr :: BStr , command : & Path , is_exe : bool) -> Option < PathBuf > { let mut path = gix_path :: try_from_bstr (root) . ok () ? . join (command) ; if ! is_exe { path . set_extension ("exe") ; } if path . is_file () { return Some (path) ; } if is_exe { return None ; } path . set_extension ("") ; path . is_file () . then_some (path) } if command . components () . take (2) . count () == 2 { return None ; } let path = gix_path :: os_str_into_bstr (path_value) . ok () ? ; let is_exe = is_exe (command) ; for root in path . split (| b | * b == b';') { if let Some (executable) = lookup (root . as_bstr () , command , is_exe) { return Some (executable) ; } } None }
    };
}

win_path_lookup!()