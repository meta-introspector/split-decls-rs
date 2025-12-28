macro_rules! find_binutils_dlltool {
    () => {
        fn find_binutils_dlltool (sess : & Session) -> OsString { assert ! (sess . target . options . is_like_windows && ! sess . target . options . is_like_msvc) ; if let Some (dlltool_path) = & sess . opts . cg . dlltool { return dlltool_path . clone () . into_os_string () ; } let tool_name : OsString = if sess . host . options . is_like_windows { "dlltool.exe" } else { match sess . target . arch . as_ref () { "x86_64" => "x86_64-w64-mingw32-dlltool" , "x86" => "i686-w64-mingw32-dlltool" , "aarch64" => "aarch64-w64-mingw32-dlltool" , _ => "dlltool" , } } . into () ; for dir in env :: split_paths (& env :: var_os ("PATH") . unwrap_or_default ()) { let full_path = dir . join (& tool_name) ; if full_path . is_file () { return full_path . into_os_string () ; } } tool_name }
    };
}

find_binutils_dlltool!();