macro_rules! stdout_to_path {
    () => {
        fn stdout_to_path (mut stdout : Vec < u8 >) -> PathBuf { if let Some (b'\n') = stdout . last () { let _ = stdout . pop () . unwrap () ; } # [cfg (unix)] let path = < OsString as std :: os :: unix :: ffi :: OsStringExt > :: from_vec (stdout) ; # [cfg (not (unix))] let path = OsString :: from (String :: from_utf8 (stdout) . expect ("stdout must be UTF-8")) ; PathBuf :: from (path) }
    };
}

stdout_to_path!();