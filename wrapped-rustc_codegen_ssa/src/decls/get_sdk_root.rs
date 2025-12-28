macro_rules! deps {
    () => {
        XcrunSdkPathWarning!();
    };
}

macro_rules! get_sdk_root {
    () => {
        deps!();
        pub (super) fn get_sdk_root (sess : & Session) -> Option < PathBuf > { let sdk_name = sdk_name (& sess . target) ; match xcrun_show_sdk_path (sdk_name , false) { Ok ((path , stderr)) => { if ! stderr . is_empty () { sess . dcx () . emit_warn (XcrunSdkPathWarning { sdk_name , stderr }) ; } Some (path) } Err (err) => { let mut diag = sess . dcx () . create_warn (err) ; diag . note (fluent :: codegen_ssa_xcrun_about) ; if let Some (developer_dir) = xcode_select_developer_dir () { diag . arg ("developer_dir" , & developer_dir) ; diag . note (fluent :: codegen_ssa_xcrun_found_developer_dir) ; if developer_dir . as_os_str () . to_string_lossy () . contains ("CommandLineTools") { if sdk_name != "MacOSX" { diag . help (fluent :: codegen_ssa_xcrun_command_line_tools_insufficient) ; } } } else { diag . help (fluent :: codegen_ssa_xcrun_no_developer_dir) ; } diag . emit () ; None } } }
    };
}

get_sdk_root!()