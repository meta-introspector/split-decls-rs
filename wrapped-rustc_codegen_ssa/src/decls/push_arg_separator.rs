macro_rules! push_arg_separator {
    () => {
        fn push_arg_separator (cpp_like_debuginfo : bool , output : & mut String) { if cpp_like_debuginfo { output . push (',') ; } else { output . push_str (", ") ; } ; }
    };
}

push_arg_separator!()