macro_rules! join_path_segments {
    () => {
        fn join_path_segments (path : & Path , sep : & str) -> String { let mut output = String :: new () ; for segment in & path . segments { if write ! (output , "{}{}" , if output . is_empty () { "" } else { sep } , segment . ident) . is_err () { break ; } ; } output }
    };
}

join_path_segments!();