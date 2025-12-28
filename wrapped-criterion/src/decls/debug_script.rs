macro_rules! debug_script {
    () => {
        fn debug_script (path : & Path , figure : & Figure) { if crate :: debug_enabled () { let mut script_path = path . to_path_buf () ; script_path . set_extension ("gnuplot") ; info ! ("Writing gnuplot script to {:?}" , script_path) ; let result = figure . save (script_path . as_path ()) ; if let Err (e) = result { error ! ("Failed to write debug output: {}" , e) ; } } }
    };
}

debug_script!()